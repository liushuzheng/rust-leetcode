use std::collections::HashMap;

///rust实现通用LRU缓存淘汰算法~写数据库组件一定用得到 
///https://mp.weixin.qq.com/s/CehwNrh2TGuKXp82HBURzQ
#[derive(Debug)]
struct LRUNode<T> {
    next: Option<Box<LRUNode<T>>>,
    prev: Option<Box<LRUNode<T>>>,
    data: Option<T>,
}

#[derive(Debug)]
struct LRUList<T> {
    head: LRUNode<T>,
    count: usize,
}

/// cache基于cachekey快速查找
type CacheKey = [u8; 16];

// 记录value和key在链表中的指针
type CacheEntry<T> = (T, LRUHandle<CacheKey>);

// 可变裸指针，方便通过裸指针修改
type LRUHandle<T> = *mut LRUNode<T>;

#[derive(Debug)]
struct Cache<T> {
    list: LRUList<CacheKey>,
    map: HashMap<CacheKey, CacheEntry<T>>,
    cap: usize,
}

impl<T> LRUList<T> {
    fn new() -> Self {
        LRUList {
            head: LRUNode {
                next: None,
                prev: None,
                data: None,
            },
            count: 0,
        }
    }

    fn insert(&mut self, data: T) -> LRUHandle<T> {
        
        
       &mut Self::new().head
    }
}