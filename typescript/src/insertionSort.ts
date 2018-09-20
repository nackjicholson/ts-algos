function insertionSort(xs: number[]) {
  xs = xs.slice();
  for (let i = 1; i < xs.length; i++) {
    const current = xs[i];
    let j = i - 1;

    while (j >= 0 && xs[j] > current) {
      xs[j + 1] = xs[j];
      j--;
    }
    xs[j + 1] = current;
  }
  return xs;
}

console.log(insertionSort([4, 3, 2, 1]));
