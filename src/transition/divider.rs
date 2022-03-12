use super::formula::{Point, Degree};

/// 緩和曲線の距離程を分割する。
///
/// - 距離程の原点(0m)は任意の場所にある。緩和曲線の始点が0mであるとは限らない。
/// - 区間の境界では距離程が整数になる。ただし初回区間の始点と最終区間の終点は小数になりうる。
pub struct Divider<'a> {
    /// 初回区間
    first: (f64, i32),

    /// 最終区間
    last: (i32, f64),

    /// 現在区間の始点
    l0: i32,

    /// 現在区間の始点までの回転角
    a0: Degree,

    /// 現在区間の始点の座標
    p0: &'a Point,
}

impl<'a> Divider<'a> {
    /// `l0` で始まり `l1` で終わる距離程の分割を表す構造体を生成する。
    /// 
    /// - `l1` よりも `l0` が大きい場合の動作は未定義。
    pub fn new(l0: f64, l1: f64, p0: &'a Point) -> Self {
        Self {
            first: (l0, l0.floor() as i32 + 1),
            last: (l1.ceil() as i32 - 1, l1),
            l0: l0 as i32,
            a0: Degree(0.0),
            p0,
        }
    }
}

impl<'a> Iterator for Divider<'a> {
    type Item = Segment;

    fn next(&mut self) -> Option<Self::Item> {
        // 区間始点
        let l0 = match self.l0 + 1 == self.first.1 {
            true => self.first.0, // 初回区間
            false => self.l0 as f64,
        };

        // 区間終点
        let l1 = match self.l0 == self.last.0 {
            true => self.last.1, // 最終区間
            false => (self.l0 + 1) as f64,
        };

        // 区間長
        let len = l1 - l0;

        // 緩和曲線始点から区間中央までの弧長
        let s = l0 - self.first.0 + (len / 2.0);

        Some(Self::Item {
            s,
            len,
            a0: self.a0, // todo: 仮
            p0: *self.p0, // todo: 仮
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = (self.last.0 - self.first.1 + 2) as usize;
        (size, Some(size))
    }
}

/// ひとつの区間
pub struct Segment {
    /// 緩和曲線始点から区間中央までの弧長
    pub s: f64,

    /// 区間長
    pub len: f64,

    /// 前回までの回転角
    pub a0: Degree,

    /// 前回の座標
    pub p0: Point,
}
