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

