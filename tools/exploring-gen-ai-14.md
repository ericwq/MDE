# 指导 LLM 编写健壮的 Java ByteBuffer 代码

<img src="img/donkey-card.png" width="25%" align="right"/></br>
本文为 [探索生成式AI](exploring-gen-ai.md) 系列的一部分，该系列记录了 Thoughtworks 技术人员在软件开发中运用生成式 AI 技术的探索实践。

|| |
|:---|---:|
|[Unmesh Joshi](https://twitter.com/unmeshjoshi)| |
|<img src="img/uj.jpg" width="30%" /> |Unmesh 是 Thoughtworks 公司的杰出工程师，常驻印度 Puna。他是 [Patterns of Distributed Systems](https://martinfowler.com/books/patterns-distributed.html) 一书的作者。|
| [原文](https://martinfowler.com/articles/exploring-gen-ai/14-guiding-llm-java-bytebuffer.html) |2025/4/17|

---
像 Cursor 这类 AI 编辑器能够借助 LLM 快速生成代码，处理样板代码，并为各类开发任务提供可用的代码片段。
但在构建高健壮性系统时，仅功能正确只是最基本的要求。
代码还必须保证安全、可预期、易于维护，且不存在难以察觉的副作用。

本文通过开发者与 LLM 的对话过程，展示专业的指导对于将一段初始存在安全隐患的代码片段改造为健壮、可投入系统使用的组件至关重要。
本次开发是为内部技术研讨会编写类 BoltDB 存储引擎这一大型开发工作的一部分。
本文同时也展示了，在基于 LLM 的开发模式下，将大型开发任务拆解为更小的编程单元会更高效。
我们将以在 Java ByteBuffer 中读写固定结构文件头这一常见任务为例展开说明。

## 场景：定义页头
我们的目标是创建一个 Java 类，用于在 ByteBuffer 中管理一个简单的页头结构。
该结构包含：页类型（byte）、页层级（byte）、键数量（short）、空闲空间偏移量（int）以及右兄弟页 ID（int）。

## 对话开始
开启对话的初始提示

<div style="background-color: #0a2463; padding: 8px; border-left: 4px solid green;">
  你能给我演示如何在 Java ByteBuffer 中读写一个简单的页头结构吗？
  该页头应包含：页类型（byte）、页层级（byte）、键数量（short）、空闲空间偏移量（int）以及右兄弟页 ID（int）。
  </br></br>
  Can you show me how to write and read a simple page header structure to a Java ByteBuffer? 
  The header should contain: Page Type (byte), Page Level (byte), Number of Keys (short), Free Space Offset (int), Right Sibling Page ID (int).
</div></br>

LLM 生成的代码（使用 position() 方法）

```
Certainly. Here's a basic example using ByteBuffer's position() method to manage reads
and writes:”
```
```java
import java.nio.ByteBuffer;

public class PageHeaderV1 {
    // Offsets (implicitly managed by position)
    private static final int PAGE_TYPE_OFFSET = 0;
    private static final int LEVEL_OFFSET = 1; // pageType is 1 byte
    private static final int NUM_KEYS_OFFSET = 2; // level is 1 byte
    private static final int FREE_SPACE_OFFSET_OFFSET = 4; // numKeys is 2 bytes
    private static final int RIGHT_SIBLING_OFFSET = 8; // freeSpaceOffset is 4 bytes
    public static final int HEADER_SIZE = 12; // rightSibling is 4 bytes

    public static void write(ByteBuffer buffer, byte pageType, byte level,
    short numKeys, int freeSpaceOffset, int rightSiblingPageId) {
        buffer.clear(); // Reset position to 0, limit to capacity

        buffer.position(PAGE_TYPE_OFFSET);
        buffer.put(pageType);

        buffer.position(LEVEL_OFFSET);
        buffer.put(level);

        buffer.position(NUM_KEYS_OFFSET);
        buffer.putShort(numKeys);

        buffer.position(FREE_SPACE_OFFSET_OFFSET);
        buffer.putInt(freeSpaceOffset);

        buffer.position(RIGHT_SIBLING_OFFSET);
        buffer.putInt(rightSiblingPageId);
    }

    public static byte getPageType(ByteBuffer buffer) {
        buffer.position(PAGE_TYPE_OFFSET);
        return buffer.get();
    }

    public static byte getLevel(ByteBuffer buffer) {
        buffer.position(LEVEL_OFFSET);
        return buffer.get();
    }

    public static short getNumKeys(ByteBuffer buffer) {
        buffer.position(NUM_KEYS_OFFSET);
        return buffer.getShort();
    }

    public static int getFreeSpaceOffset(ByteBuffer buffer) {
        buffer.position(FREE_SPACE_OFFSET_OFFSET);
        return buffer.getInt();
    }

    public static int getRightSiblingPageId(ByteBuffer buffer) {
        buffer.position(RIGHT_SIBLING_OFFSET);
        return buffer.getInt();
    }
}
```
```
This code uses position() to move to the correct offset before each read or write.
```

**开发者洞见** ：LLM 针对简单场景生成了功能正确的代码。
然而，专业开发者会立刻察觉到一个潜在问题：对 `buffer.position()` 方法的使用。
该方法会修改 ByteBuffer 对象自身的状态。

## 开发者建议使用绝对偏移量

<div style="background-color: #0a2463; padding: 8px; border-left: 4px solid green;">
  谢谢。不过，频繁修改缓冲区的位置可能会引发问题，尤其是在使用 MappedByteBuffer 这类共享缓冲区或并发代码时。
  这种做法会引入副作用。
  我们能否改用绝对定位的 get(index) 和 put(index, value) 方法来重写这段代码，从而使其不依赖缓冲区的位置状态？
</br></br>
  Thanks. However, constantly changing the buffer's position can cause issues, especially with shared buffers like MappedByteBuffer or in concurrent code. 
  It introduces side effects. 
  Can we rewrite this using the absolute get(index) and put(index, value) methods instead to make it stateless regarding the buffer's position?
</div></br>

**开发者洞见** ：开发者识别出了具体问题（通过状态变更产生的副作用），并理解该问题在何种场景下会变得危险（共享缓冲区、并发场景）。
他们引导 LLM 采用更安全、无状态的编码模式。

```
You are absolutely right. Using absolute offset methods avoids modifying the
buffer's position state, which is much safer for shared or concurrent use cases.
Here's how you can read the numKeys and freeSpaceOffset using the absolute methods:
```
```java
import java.nio.ByteBuffer;

// Example correction for a couple of methods:
public class PageHeaderCorrection {
    private static final int NUM_KEYS_OFFSET = 2;
    private static final int FREE_SPACE_OFFSET_OFFSET = 4;

    // Corrected getter using absolute offset
    public static short getNumKeys(ByteBuffer buffer) {
        return buffer.getShort(NUM_KEYS_OFFSET);
        // Reads at index 2, doesn't change position
    }

    // Corrected getter using absolute offset
    public static int getFreeSpaceOffset(ByteBuffer buffer) {
        return buffer.getInt(FREE_SPACE_OFFSET_OFFSET);
        // Reads at index 4, doesn't change position
    }

    // Similarly, writing would use put(index, value):
    public static void writeNumKeys(ByteBuffer buffer, short numKeys) {
         buffer.putShort(NUM_KEYS_OFFSET, numKeys);
         // Writes at index 2, doesn't change position
    }
}
```
```
Using get(index) and put(index, value) ensures the operations are side-effect-free
concerning the buffer's position.
```

**开发者洞见** ：LLM 正确实现了所建议的模式。
开发者现在进一步追求更优的软件设计，从静态工具方法转向面向对象的实现方式。

## 开发者要求重构为基于实例的类
<div style="background-color: #0a2463; padding: 8px; border-left: 4px solid green;">
  开发者：“非常好，这正是我们需要的模式。
  请提供完整的 PageHeader 类，所有字段均只使用绝对偏移量方法（get(index)、put(index, value)、getShort(index)、putShort(index, value) 等）。
  另外，我们来重构一下。
  不要使用静态方法，而是将 PageHeader 设计为在构造函数中接收 ByteBuffer 的类。
  它应作为该缓冲区片段的视图或包装器。
  去掉接收所有属性的静态 write 方法，改为提供实例方法，直接对构造函数中传入的缓冲区进行各字段的读写，并且依然使用安全的绝对偏移量方法。”
</br></br>
  Developer: “Excellent, that's exactly the pattern needed. Please provide the complete PageHeader class using only the absolute offset methods (get(index), put(index, value), getShort(index), putShort(index, value), etc.) for all fields. 
  Also, let's refactor this. Instead of static methods, make PageHeader a class that takes the ByteBuffer in its constructor.
  It should act as a view or wrapper around that buffer section. Remove the static write method that takes all attributes.
  Instead, provide instance methods for getting and setting each field directly on the buffer provided in the constructor, still using the safe absolute offset methods.”
</div></br>

**开发者洞见** ：开发者运用了软件设计原则：封装（对缓冲区进行包装）与面向对象设计（让实例方法操作对象状态——即缓冲区引用）。
这提升了代码的易用性与可维护性。

```
LLM: “Understood. Refactoring the PageHeader into a class that wraps a ByteBuffer provided
at construction is a good way to encapsulate header access for a specific page buffer.
Here's the refactored class with instance methods for getting and setting fields using
absolute offsets:”
```
```java
import java.nio.ByteBuffer;

public class PageHeader {
    // Define constants for offsets and size
    private static final int OFFSET_PAGE_TYPE = 0;
    private static final int OFFSET_LEVEL = 1;
    private static final int OFFSET_NUM_KEYS = 2;
    private static final int OFFSET_FREE_SPACE = 4;
    private static final int OFFSET_RIGHT_SIBLING = 8;
    public static final int HEADER_SIZE = 12; // Total size of the header

    private final ByteBuffer buffer; // Store the buffer instance

    /**
     * Creates a PageHeader view wrapping the given ByteBuffer.
     * Assumes the buffer's relevant section starts at position 0
     * or that the buffer slice/view is correctly positioned.
     * @param buffer The ByteBuffer containing the page data.
     */
    public PageHeader(ByteBuffer buffer) {
        // Consider adding checks: buffer != null, buffer.capacity() >= HEADER_SIZE
        this.buffer = buffer;
    }

    // --- Getters using absolute offsets ---

    public byte getPageType() {
        return buffer.get(OFFSET_PAGE_TYPE);
    }

    public byte getLevel() {
        return buffer.get(OFFSET_LEVEL);
    }

    public short getNumKeys() {
        return buffer.getShort(OFFSET_NUM_KEYS);
    }

    public int getFreeSpaceOffset() {
        return buffer.getInt(OFFSET_FREE_SPACE);
    }

    public int getRightSiblingPageId() {
        return buffer.getInt(OFFSET_RIGHT_SIBLING);
    }

    // --- Setters using absolute offsets ---

    public void setPageType(byte pageType) {
        buffer.put(OFFSET_PAGE_TYPE, pageType);
    }

    public void setLevel(byte level) {
        buffer.put(OFFSET_LEVEL, level);
    }

    public void setNumKeys(short numKeys) {
        buffer.putShort(OFFSET_NUM_KEYS, numKeys);
    }

    public void setFreeSpaceOffset(int freeSpaceOffset) {
        buffer.putInt(OFFSET_FREE_SPACE, freeSpaceOffset);
    }

    public void setRightSiblingPageId(int rightSiblingPageId) {
        buffer.putInt(OFFSET_RIGHT_SIBLING, rightSiblingPageId);
    }

    /**
     * Initializes a new header structure with default/zero values
     * in the underlying buffer.
     */
    public void initializeNewHeader() {
        setPageType((byte) 0);
        setLevel((byte) 0);
        setNumKeys((short) 0);
        setFreeSpaceOffset(HEADER_SIZE); // Default free space starts after header
        setRightSiblingPageId(0); // Assuming 0 represents no sibling initially
        // Or use specific constants for null/invalid IDs if available
    }
}
```
```
This version encapsulates access to the header fields within an instance tied to
a specific ByteBuffer. All read and write operations use absolute offsets, ensuring
safety and avoiding side effects on the buffer's position. I've also added an example
initializeNewHeader method for convenience.
```

## 要点总结：开发者角色的重要性
仍有更多可优化的空间。但这段对话凸显了在使用 LLM 时，开发者专业能力不可或缺的关键方面：

1. **识别潜在风险** ：初始代码虽能运行，但开发者意识到在更广泛的系统场景中，使用 position() 方法带来的副作用存在隐性危险。

2. **上下文感知能力** ：理解该写法对 MappedByteBuffer、并发编程及库设计的影响，是摒弃有状态实现方案的关键。

3. **引导采用健壮模式** ：开发者凭借底层安全编程实践的深厚知识，明确提出使用无状态的绝对偏移量方法。

4. **践行软件设计原则** ：最终重构不仅追求功能正确，更注重更优设计 —— 封装、基于实例的逻辑、可维护性（常量定义）与易用性。

5. **批判性评估** ：整个过程中，开发者不仅从功能需求层面，还从安全性、稳定性、可维护性等非功能需求维度，对 LLM 的输出进行了批判性评估。

## 结论
LLM 是极为强大的编码助手，能够加速开发进程并处理复杂任务。
但正如本案例研究所示，它们是需要依靠引导才能发挥作用的工具。
构建健壮、可靠且高性能的系统，需要经验丰富的开发者具备批判性思维、上下文理解能力以及深厚的系统知识。
<ins>专业开发者并非仅通过提示词获取代码，而是会开展评估、引导、优化与集成工作，确保最终成果满足现实软件工程的严苛要求</ins>。
