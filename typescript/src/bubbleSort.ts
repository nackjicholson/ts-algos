function bubbleSortConcept(array: number[]): number[] {
  array = array.slice();
  for (let i = 0; i < array.length; i++) {
    for (let j = 0; j < array.length - 1; j++) {
      if (array[j] > array[j + 1]) {
        let left = array[j];
        let right = array[j + 1];
        array[j] = right;
        array[j + 1] = left;
      }
      console.log(array);
    }
  }
  return array;
}


function bubbleSort(array: number[]): number[] {
  array = array.slice();

  while (true) {
    let swapped = false;
    for (let i = 0; i < array.length - 1; i++) {
      if (array[i] > array[i + 1]) {
        [array[i], array[i + 1]] = [array[i + 1], array[i]];
        swapped = true;
      }
    }
    console.log(array);
    if (!swapped) {
      break;
    }
  }

  return array;
}


const array = [10, 9, 8,  7, 6, 5, 4, 3, 2, 1];

console.log('concept implementation:');
console.log(bubbleSortConcept(array));
console.log('O(N^2) implementation:');
console.log(bubbleSort(array));
