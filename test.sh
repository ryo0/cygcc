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

assert 47 '5 + 6 * 7;'
assert 15 '5 * (9 - 6);'
assert 4 '(3 + 5) / 2 ;'
assert 10 '-10+20;'
assert 10 '- -10;'
assert 10 '- - +10;'
assert 24 '1 + 2 * 3 / 2 + 4 * 5;'

assert 0 '0==1;'
assert 1 '42==42;'
assert 1 '0!=1;'
assert 0 '42!=42;'

assert 1 '0<1;'
assert 0 '1<1;'
assert 0 '2<1;'
assert 1 '0<=1;'
assert 1 '1<=1;'
assert 0 '2<=1;'

assert 1 '1>0;'
assert 0 '1>1;'
assert 0 '1>2;'
assert 1 '1>=0;'
assert 1 '1>=1;'
assert 0 '1>=2;'

assert 1 '1 == 1;'  
assert 0 '1 != 1;'  
assert 0 '-1 * 3 != -3;'  
assert 1 '-1 * 3 == -3;'  
assert 1 '1 + 2 * 3 * 2 + 4 * -5 == -4 + -3;'
assert 1 '3 > 0;'  
assert 0 '3 > 3;'  
assert 1 '3 >= 3;'  
assert 1 '1 - 1 * 2 + 3 < 2*2;'  
assert 0 '1 < 1;'  
assert 1 '1 <= 1;' 

assert 3 'a = 3; a;'  
assert 4 'a = b = 2; a + b;'  
assert 8 'a = 3; z = 5; a + z;'  
assert 8 'foo=3; bar = 5; foo+bar;' 
assert 15 'foo= bar = hoge = 5; foo+bar+hoge;' 
assert 8 'foo= bar = 2; 3*foo+bar;' 
assert 21 'a = 3; foo= bar = hoge = a * 2; foo + bar + hoge + a;' 
echo OK