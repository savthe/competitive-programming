#include <iostream>

using namespace std;

using ull = uint64_t;

ull a, k, b, m, x;

bool test(ull days) {
    ull s = a * (days - (days / k));
    s += b * (days - (days / m));

    return s >= x;
}

int main() {
    cin >> a >> k >> b >> m >> x;

    ull low = 0, high = 1;
    //high = 1e18 может вызвать переполнение в a*high,
    //поэтому ищем адекватную верхнюю грань
    while (!test(high)) high *= 2;

    while (low < high) {
        ull mid = (low + high) / 2;
        if (!test(mid)) {
            low = mid + 1;
        }
        else {
            high = mid;
        }
    }

    cout << low << endl;
}

