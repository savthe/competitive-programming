#include <iostream>
#include <cassert>

using namespace std;

int main() {
    int64_t a, b, c, d;
    cin >> a >> b >> c >> d;

    auto f = [&](double x) {
        return a*x*x*x + b*x*x + c*x + d;
    };

    double low = -1e9, high = 1e9;
    assert(f(low) * f(high) <= 0);
    while (high - low >= 0.00001) {
        double m = (low + high) / 2;
        if (f(low) * f(m) <= 0) high = m;
        else low = m;
    }
    cout << (low + high) / 2;
}

