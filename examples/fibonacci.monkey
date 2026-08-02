// Define a fibonacci function and compute fibonacci(10)
let fibonacci = fn(x) {
  if (x == 0) {
    return 0;
  } else {
    if (x == 1) {
      return 1;
    } else {
      return fibonacci(x - 1) + fibonacci(x - 2);
    }
  }
};

fibonacci(10);
