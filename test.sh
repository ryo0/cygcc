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

# assert 47 '5 + 6 * 7;'
# assert 15 '5 * (9 - 6);'
# assert 4 '(3 + 5) / 2 ;'
# assert 10 '-10+20;'
# assert 10 '- -10;'
# assert 10 '- - +10;'
# assert 24 '1 + 2 * 3 / 2 + 4 * 5;'

# assert 0 '0==1;'
# assert 1 '42==42;'
# assert 1 '0!=1;'
# assert 0 '42!=42;'

# assert 1 '0<1;'
# assert 0 '1<1;'
# assert 0 '2<1;'
# assert 1 '0<=1;'
# assert 1 '1<=1;'
# assert 0 '2<=1;'

# assert 1 '1>0;'
# assert 0 '1>1;'
# assert 0 '1>2;'
# assert 1 '1>=0;'
# assert 1 '1>=1;'
# assert 0 '1>=2;'

# assert 1 '1 == 1;'  
# assert 0 '1 != 1;'  
# assert 0 '-1 * 3 != -3;'  
# assert 1 '-1 * 3 == -3;'  
# assert 1 '1 + 2 * 3 * 2 + 4 * -5 == -4 + -3;'
# assert 1 '3 > 0;'  
# assert 0 '3 > 3;'  
# assert 1 '3 >= 3;'  
# assert 1 '1 - 1 * 2 + 3 < 2*2;'  
# assert 0 '1 < 1;'  
# assert 1 '1 <= 1;' 

# assert 3 'a = 3; a;'  
# assert 4 'a = b = 2; a + b;'  
# assert 8 'a = 3; z = 5; a + z;'  
# assert 8 'foo=3; bar = 5; foo+bar;' 
# assert 15 'foo= bar = hoge = 5; foo+bar+hoge;' 
# assert 8 'foo= bar = 2; 3*foo+bar;' 
# assert 21 'a = 3; foo= bar = hoge = a * 2; foo + bar + hoge + a;' 


# assert 3 'a=3; return a;'
# assert 8 'a=3; z=5; return a+z;'

# assert 3 'a=3; return a;'
# assert 8 'a=3; z=5; return a+z;'
# assert 6 'a=b=3; return a+b;'
# assert 3 'foo=3; return foo;'
# assert 8 'foo123=3; bar=5; return foo123+bar;'

# assert 1 'return 1; 2; 3;'
# assert 2 '1; return 2; 3;'
# assert 3 '1; 2; return 3;'

# assert 3 'if (0) {return 2;} return 3; '
# assert 3 'if (1-1){ return 2;} return 3; '
# assert 2 'if (1) return 2; return 3; '
# assert 2 'if (2-1) return 2; return 3; '

# assert 3 'if (0) {return 2;} else {return 3;} '
# assert 3 'if (1-1) return 2; else return 3; '
# assert 2 'if (1) return 2; else return 3; '
# assert 2 'if (2-1) {return 2;} else {return 3;} '

# assert 10 'i=0; while(i<10) {i=i+1;} return i; '
# assert 35 ' i=0; j = 7; while(i<35) {i= i + j;} return i;'

# assert 60 'sum=0; for (i=10; i<15; i=i+1){ sum = sum + i;} return sum;'

# assert 3 'i = 1; j = 2; j = i + j; return j;'
# assert 2 'j = 0; j = 2; return j;'

# assert 11 'i=0; j=0; for (i=0; i<=10; i=i+1){ j=i+j; } return i; '
# assert 2 'i=0; j=0; for (i=0; i<=10; i=i+1) { j=2;  }return j; '

# assert 11 'i=0; j=0; for (i=0; i<=10; i=i+1) { j=2; } return i; '
# assert 55 'i=0; j=0; for (i=0; i<=10; i=i+1) {j=i+j;} return j; '

# assert 9 'j = 0; for (i=0; i<10; i=i+1) {j = i;} return j; '
# assert 10 'j = 0; for (i=0; i<10; i=i+1) {j = i; }return i;'

# assert 3 ' for (;;) {return 3;} return 5; '

# assert 90 'j = 0; for (i = 0; i < 10; i = i + 1) {j = j + i ; j = j + i;} return j; '

# assert 3 'if (0) {return 2;} else {i = 3; return i; return 100;} '


assert 32 'int main() { return ret32(); } int ret32() { return 32; }'
assert 7 'int main() { return add2(3,4); } int add2(x, y) { return x+y; }'
assert 1 'int main() { return sub2(4,3); } int sub2( x, i y) { return x-y; }'
assert 55 'int main() { return fib(9); } int fib(x) { if (x<=1) return 1; return fib(x-1) + fib(x-2); }'

echo OK
