// メモリアドレスを表す型
// Rustはメモリ消費を最小にするように設計されているので、値はデフォルトでネストする
// 結果として、ある値から別の値を指す場合には、必ず明示的にポインタを用いなければならない

// 参照
// 参照は、スタック上の値でも、ヒープ上の値でも、どこにある値でも指すことができる
// コンパイラが値の所有権と生存期間を管理している　コンパイル時に、ダングリングポインタや、多重フリー、ポインタの無効化を取り除くことができる

// Box
// ヒープ上に値を確保する
fn main() {
    let t = (12, "eggs");
    // スコープから外れると、メモリは即座に解法される
    let _b = Box::new(t); // allocate a tuple in the heap
}

// raw pointer