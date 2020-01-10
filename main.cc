#include <iostream>
#include <vector>
#include <algorithm>
#include <cmath>
#include <utility>
#include <map>

#include <stdio.h>

#define rep(i, n) for (int i = 0; i < (n); i++)
#define irep(i, k, n) for (int i = (k); i < (n); i++)
#define rrep(i, k, n) for (int i = (k); i >= (n); i++)
#define pb push_back
#define eb emplace_back
#define vec vector

typedef uint8_t   i8;
typedef uint16_t  i16;
typedef int       i32;
typedef long long i64;

template<class T> bool chmin(T& a, T b) {
  if (a > b) {
    a = b;
    return true;
  }
  return false;
}

template<class T> bool chmax(T& a, T b) {
  if (a < b) {
    a = b;
    return true;
  }
  return false;
}

template<class T> T gcd(T m, T n) {
  if (n == 0) return m;
  return gcd(n, m % n);
}

template<class T> T lcm(T m, T n) {
  return m * n / gcd(m, n);
}

const i64 inf = 1LL << 60;
const i32 mod = 1e9 + 7;

using namespace std;

/* Input variables. */


/* Solve! */
void solve() {
}

int main() {
  /* Input handling. */

  solve();
  return 0;
}
