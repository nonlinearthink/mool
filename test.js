let obj = {
  test: { func: () => this },
  func2: n => {
    console.log(n);
    return function (o) {
      return obj.func2(n);
    };
  }
};
obj.test.func();
obj.test.func.call(this);
var a = obj.func2(1);
a(1);
a(2);
a(3);
