#!/bin/bash
assert() {
  expected="$1"
  input="$2"
  ./target/debug/cygcc "$input" > tmp.s
  cc -o tmp tmp.s
  ./tmp
  actual="$?"

  if [ "$actual" = "$expected" ]; then
    echo "$input => $actual"
  else
    echo "$input => $expected expected, but got $actual"
    exit 1
  fi
}

assert 7 'int main() { int x; int y; int *z; x=3; y=5; z=&y; *(z+1)=7; return x; }'

assert 32 'int main() { return ret32(); } int ret32() { return 32; }'

assert 47 'int main(){ return 5 + 6 * 7; }'
assert 15 'int main(){ return 5 * (9 - 6); }'
assert 4 'int main(){ return (3 + 5) / 2 ; }'
assert 10 'int main(){ return -10+20; }'
assert 10 'int main(){ return - -10; }'
assert 10 'int main(){ return - - +10; }'
assert 24 'int main(){ return 1 + 2 * 3 / 2 + 4 * 5; }'

assert 0 'int main(){ return 0==1; }'
assert 1 'int main(){ return 42==42; }'
assert 1 'int main(){ return 0!=1; }'
assert 0 'int main(){ return 42!=42; }'

assert 1 'int main(){ return 0<1; }'
assert 0 'int main(){ return 1<1; }'
assert 0 'int main(){ return 2<1; }'
assert 1 'int main(){ return 0<=1; }'
assert 1 'int main(){ return 1<=1; }'
assert 0 'int main(){ return 2<=1; }'

assert 1 'int main(){ return 1>0; }'
assert 0 'int main(){ return 1>1; }'
assert 0 'int main(){ return 1>2; }'
assert 1 'int main(){ return 1>=0; }'
assert 1 'int main(){ return 1>=1; }'
assert 0 'int main(){ return 1>=2; }'

assert 1 'int main(){ return 1 == 1; }'  
assert 0 'int main(){ return 1 != 1; }'  
assert 0 'int main(){ return -1 * 3 != -3; }'  
assert 1 'int main(){ return -1 * 3 == -3; }'  
assert 1 'int main(){ return 1 + 2 * 3 * 2 + 4 * -5 == -4 + -3; }'
assert 1 'int main(){ return 3 > 0; }'  
assert 0 'int main(){ return 3 > 3; }'  
assert 1 'int main(){ return 3 >= 3; }'  
assert 1 'int main(){ return 1 - 1 * 2 + 3 < 2*2; }'  
assert 0 'int main(){ return 1 < 1; }'  
assert 1 'int main(){ return 1 <= 1; }' 

assert 3 'int main(){ int a; return a=3; }'
assert 3 'int main(){int a;  a=3; return 3; }'
assert 3 'int main(){ int a; a = 3; return a; }'  
assert 2 'int main(){ int a; int b;  a = b = 2;return 2; }'  
assert 2 'int main(){int a; int b; a = b = 2;return a; }'  
assert 4 'int main(){ int a; int b;  a = b = 2;return a + b; }' 
assert 6 'int main(){ int a; a = 3; return a + a; }'
assert 6 'int main(){ int a; int z; a = 3; z = 5; return a+a; }'
assert 10 'int main(){ int a; int z;  a=3; z=5; return z+z; }'
assert 8 'int main(){ int a; int z; a = 3; z = 5; return a + z; }'  
assert 8 'int main(){ int foo; int bar; foo=3; bar = 5; return foo+bar; }' 
assert 15 'int main(){ int foo; int bar; int hoge; foo= bar = hoge = 5; return foo+bar+hoge; }' 
assert 8 'int main(){int foo; int bar; foo= bar = 2; return 3*foo+bar; }' 
assert 21 'int main(){ int a; int foo; int bar; int hoge; a = 3; foo= bar = hoge = a * 2;return foo + bar + hoge + a; }' 

assert 3 'int main(){ int a; a=3; return a; }'
assert 8 'int main(){  int a; int z; a=3; z=5; return a+z; }'

assert 3 'int main(){ int a; a=3; return a;}'
assert 8 'int main(){ int a; int z; a=3; z=5; return a+z;}'
assert 6 'int main(){ int a; int b; a=b=3; return a+b;}'
assert 3 'int main(){ int foo; foo=3; return foo;}'
assert 8 'int main(){ int foo123; int bar; foo123=3; bar=5; return foo123+bar;}'

assert 1 'int main(){ return 1; 2; 3;}'
assert 2 'int main(){ 1; return 2; 3;}'
assert 3 'int main(){ 1; 2; return 3;}'

assert 3 'int main(){ if (0) {return 2;} return 3; }'
assert 3 'int main(){ if (1-1){ return 2;} return 3; }'
assert 2 'int main(){ if (1) return 2; return 3; }'
assert 2 'int main(){ if (2-1) return 2; return 3; }'

assert 3 'int main(){ if (0) {return 2;} else {return 3;} '
assert 3 'int main(){ if (1-1) return 2; else return 3; }'
assert 2 'int main(){ if (1) return 2; else return 3; }'
assert 2 'int main(){ if (2-1) {return 2;} else {return 3;} '

assert 10 'int main(){ int i; i=0; while(i<10) {i=i+1;} return i; }'
assert 35 'int main(){ int i; int j; i=0; j = 7; while(i<35) {i= i + j;} return i;}'

assert 60 'int main(){ int sum; int i; sum=0; for (i=10; i<15; i=i+1){ sum = sum + i;} return sum;}'

assert 3 'int main(){ int i; int j; i = 1; j = 2; j = i + j; return j;}'
assert 2 'int main(){ int j;  int i; int j; j = 0; j = 2; return j;}'

assert 11 'int main(){ int i; int j;  i=0; j=0; for (i=0; i<=10; i=i+1){ j=i+j; } return i; }'
assert 2 'int main(){  int i; int j; i=0; j=0; for (i=0; i<=10; i=i+1) { j=2;  }return j; }'

assert 11 'int main(){ int i; int j;  i=0; j=0; for (i=0; i<=10; i=i+1) { j=2; } return i; }'
assert 55 'int main(){  int i; int j; i=0; j=0; for (i=0; i<=10; i=i+1) {j=i+j;} return j; }'

assert 9 'int main(){  int i; int j; j = 0; for (i=0; i<10; i=i+1) {j = i;} return j; }'
assert 10 'int main() int i; int j; { j = 0; for (i=0; i<10; i=i+1) {j = i; }return i;}'

assert 3 'int main(){  for (;;) {return 3;} return 5; }'

assert 90 'int main(){ int i; int j; j = 0; for (i = 0; i < 10; i = i + 1) {j = j + i ; j = j + i;} return j; }'

assert 3 'int main(){ if (0) {return 2;} else {int i; i = 3; return i; return 100;} '

assert 32 'int main() { return ret32(); } int ret32() { return 32; }'
assert 7 'int main() { return add2(3,4); } int add2(int x, int y) { return x+y; }'
assert 1 'int main() { return sub2(4,3); } int sub2(int x,int y) { return x-y; }'
assert 55 'int main() { return fib(9); } int fib(int x) { if (x<=1) return 1; return fib(x-1) + fib(x-2); }'

assert 3 'int main() { int a; a = 3; return fib(a); } int fib(int x) { if (x<=1) return 1; return fib(x-1) + fib(x-2); }'

assert 8 'int main() { return add2_plus1(3,4); } int add2_plus1(int x, int y) { int p; p = 1; return x+y +p; }'

assert 5 'int main() { int x; int y; x=3; y=5; return *(&x-8); }'
assert 3 'int main() { int x; int y; x=3; y=5; return *(&y+8); }'
assert 5 'int main() { int x; int y; x=3; y=5; return *(&x+-8); }'

assert 7 'int main() { int x; int y; x=3; y=5; *(&x-8)=7; return y; }'
assert 7 'int main() { int x; int y; x=3; y=5; *(&y+8)=7; return x; }'
assert 5 'int main() { int x; x=3; return (&x+2)-&x+3; }'

assert 3 'int main() { int x; int *y; x = 0; y=&x; *y = 3; return x; }'

assert 3 'int main() { int x; int *y; int **z;  x = 0; y=&x; z = &y; **z = 3; return x; }'
echo OK
