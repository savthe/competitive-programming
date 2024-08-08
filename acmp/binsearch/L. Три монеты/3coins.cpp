#include <iostream>
#include <map>
#include <vector>
using namespace std;

using ull = uint64_t;
ull res[3] = {0, 0, 0};


ull get(ull n) {
    static map<ull, ull> cache;

    if (cache.count(n) == 0) {
        cout << "? " << n << endl;
        ull s; cin >> s;
        cache[n] = s;
    }
    return cache[n];
}

ull sum(ull n) {
    ull s = (1 + n)*n / 2;
    for (int i = 0; i < 3; ++i)
        if (res[i] <= n) s -= res[i];
    return s;
}

ull sum(ull a, ull b) {
    return sum(b) - sum(a - 1);
}


void solve2(ull low, ull high) {
    ull m = (low + high) / 2;
    ull sm = get(m);
    if (sm == sum(m)) {
        solve2(m + 1, high);
    }
    else {
        ull shigh = get(high);
        shigh -= sm;
        if (shigh == sum(m+1, high)) {
            solve2(low, m);
        }
        else {
            res[1] = sum(m) - sm;
            res[2] = sum(m + 1, high) - shigh;
            return;
        }
    }
}

void solve3(ull low, ull high) {
    ull m = (low + high) / 2;
    ull sm = get(m);
    if (sm == sum(m)) {
        solve3(m + 1, high);
    }
    else {
        ull shigh = get(high);
        shigh -= sm; // sum in [m+1, high]
        if (shigh == sum(m+1, high)) {
            solve3(low, m);
        }
        else {
            if (sum(m + 1, high) - shigh <= high) {
                res[0] = sum(m + 1, high) - shigh;
                solve2(low, m);
            }
            else {
                res[0] = sum(m) - sm;
                solve2(m + 1, high);
            }
        }
    }
}

int main() {
    ull n; cin >> n;
    solve3(1, n);
    cout << "! " << res[0] << " " << res[1] << " " << res[2] << endl;
}

