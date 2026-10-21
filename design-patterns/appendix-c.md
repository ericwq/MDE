# 附录 C 基础库

本附录记录我们在若干设计模式的 C++ 示例代码中所使用的基础类。
我们有意让这些类保持简单、尽量精简。
我们描述以下这些类：

- `List`，一个有序的对象列表。
- `Iterator`，按顺序 (sequence) 访问聚合中各对象的接口。
- `ListIterator`，用于遍历 `List` 的迭代器。
- `Point`，一个二维点。
- `Rect`，一个轴对齐的矩形。

一些较新的 C++ 标准类型未必在所有编译器上都可用。
尤其是，如果你的编译器没有定义 `bool`，就手动定义它：

```cpp
typedef int bool;
const int true = 1;
const int false = 0;
```

## C.1 List

`List` 类模板提供一个基本容器，用于存储有序的对象列表。
`List` 按值存储元素，这意味着它既适用于内置类型，也适用于类的实例。
例如，`List<int>` 声明一个 `int` 的列表。
但本书大多数模式都用 `List` 存储指向对象的指针，例如 `List<Glyph*>`。
这样一来，`List` 就可以用于异质列表。

为方便起见，`List` 还为栈操作提供了一组别名，这使把 `List` 当作栈使用的代码更明确，而无需另外定义一个类。

```cpp
template <class Item>
class List {
public:
    List(long size = DEFAULT_LIST_CAPACITY);
    List(List&);
    ~List();
    List& operator=(const List&);

    long Count() const;
    Item& Get(long index) const;
    Item& First() const;
    Item& Last() const;
    bool Includes(const Item&) const;

    void Append(const Item&);
    void Prepend(const Item&);

    void Remove(const Item&);
    void RemoveLast();
    void RemoveFirst();
    void RemoveAll();

    Item& Top() const;
    void Push(const Item&);
    Item& Pop();
};
```

以下各小节更详细地描述这些操作。

### 构造、析构、初始化与赋值

- `List(long size)`：初始化列表。`size` 参数是对初始元素数量的提示。
- `List(List&)`：覆盖默认的拷贝构造函数，以便成员数据被正确初始化。
- `~List()`：释放列表的内部数据结构，但不释放列表中的元素。该类并非为支持子类化而设计，因此析构函数不是虚函数。
- `List& operator=(const List&)`：实现赋值操作，以正确地赋值成员数据。

### 访问

这些操作提供对列表元素的基本访问。

- `long Count() const`：返回列表中对象的个数。
- `Item& Get(long index) const`：返回给定索引处的对象。
- `Item& First() const`：返回列表中的第一个对象。
- `Item& Last() const`：返回列表中的最后一个对象。

### 添加

- `void Append(const Item&)`：把参数加入列表，使其成为最后一个元素。
- `void Prepend(const Item&)`：把参数加入列表，使其成为第一个元素。

### 移除

- `void Remove(const Item&)`：从列表中移除给定元素。该操作要求列表中元素的类型支持用于比较的 `==` 运算符。
- `void RemoveFirst()`：从列表中移除第一个元素。
- `void RemoveLast()`：从列表中移除最后一个元素。
- `void RemoveAll()`：移除列表中的所有元素。

### 栈接口

- `Item& Top() const`：返回栈顶元素（当把 `List` 视为栈时）。
- `void Push(const Item&)`：把元素压入栈中。
- `Item& Pop()`：弹出栈顶元素。

## C.2 Iterator

`Iterator` 是一个抽象类，为聚合定义遍历接口。

```cpp
template <class Item>
class Iterator {
public:
    virtual void First() = 0;
    virtual void Next() = 0;
    virtual bool IsDone() const = 0;
    virtual Item CurrentItem() const = 0;
protected:
    Iterator();
};
```

这些操作的作用如下：

- `virtual void First()`：把迭代器定位到聚合中的第一个对象。
- `virtual void Next()`：把迭代器定位到序列中的下一个对象。
- `virtual bool IsDone() const`：当序列中没有更多对象时返回 `true`。
- `virtual Item CurrentItem() const`：返回序列中当前位置上的对象。

## C.3 ListIterator

`ListIterator` 实现 `Iterator` 接口，用以遍历 `List` 对象。
它的构造函数接受一个要遍历的列表作为参数。

```cpp
template <class Item>
class ListIterator : public Iterator<Item> {
public:
    ListIterator(const List<Item>* aList);

    virtual void First();
    virtual void Next();
    virtual bool IsDone() const;
    virtual Item CurrentItem() const;
};
```

## C.4 Point

`Point` 表示二维笛卡尔坐标空间中的一个点。
`Point` 支持一些最基本的向量运算。
`Point` 的坐标定义为：

```cpp
typedef float Coord;
```

`Point` 的操作不言自明。

```cpp
class Point {
public:
    static const Point Zero;

    Point(Coord x = 0.0, Coord y = 0.0);

    Coord X() const;      void X(Coord x);
    Coord Y() const;      void Y(Coord y);

    friend Point operator+(const Point&, const Point&);
    friend Point operator-(const Point&, const Point&);
    friend Point operator*(const Point&, const Point&);
    friend Point operator/(const Point&, const Point&);

    Point& operator+=(const Point&);
    Point& operator-=(const Point&);
    Point& operator*=(const Point&);
    Point& operator/=(const Point&);

    Point operator-();

    friend bool operator==(const Point&, const Point&);
    friend bool operator!=(const Point&, const Point&);

    friend ostream& operator<<(ostream&, const Point&);
    friend istream& operator>>(istream&, Point&);
};
```

静态成员 `Zero` 表示 `Point (0, 0)`。

## C.5 Rect

`Rect` 表示一个轴对齐的矩形。
`Rect` 由一个原点和一个范围（即宽和高）定义。
`Rect` 的操作不言自明。

```cpp
class Rect {
public:
    static const Rect Zero;

    Rect(Coord x, Coord y, Coord w, Coord h);
    Rect(const Point& origin, const Point& extent);

    Coord Width() const;      void Width(Coord);
    Coord Height() const;     void Height(Coord);
    Coord Left() const;       void Left(Coord);
    Coord Bottom() const;     void Bottom(Coord);

    Point& Origin() const; void Origin(const Point&);
    Point& Extent() const; void Extent(const Point&);
    void MoveTo(const Point&);
    void MoveBy(const Point&);

    bool IsEmpty() const;
    bool Contains(const Point&) const;
};
```

静态成员 `Zero` 等价于矩形

```cpp
Rect(Point(0, 0), Point(0, 0));
```
