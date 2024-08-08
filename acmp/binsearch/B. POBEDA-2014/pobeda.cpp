#include <iostream>
#include <cmath>
using namespace std;

int main() {
    uint64_t a, b, c, d;
    cin >> a >> b >> c >> d;
    auto n = min(a, b) + min(c, d);
    uint64_t low = 0, high = 2e9;

    high = 1;
    while (high * high < n) high *= 2;

    while (low < high) {
        auto m = (low + high + 1) / 2;
        if (m*m > n) {
            high = m - 1;
        }
        else {
            low = m;
        }
    }

    cout << low << endl;
}
    /*
    auto n = min(a, b) + min(c, d);
    cout << uint64_t(sqrt(n)) << endl;
    */

