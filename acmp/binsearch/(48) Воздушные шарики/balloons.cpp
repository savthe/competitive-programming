#include <iostream>
#include <vector>

using namespace std;

using ull = uint64_t;

int m, n;
vector<int> t, y, z;

// возвращает количество шариков, которые надует
// i-й надуватель за время tm
ull count_balloons(int i, ull tm) {
    // k - полные циклы надувания с отдыхом
    ull k = tm / (t[i]*z[i] + y[i]);
    ull c = k * z[i];

    // то, что осталось -- без отдыха
    tm = tm % (t[i]*z[i] + y[i]);
    if (tm >= t[i]*z[i]) {
        c += z[i];
    }
    else {
        c += tm / t[i];
    }
    return c;
} 

bool test(ull tm) {
    ull c = 0;
    for (int i = 0; i < n; ++i) {
        c += count_balloons(i, tm);
    }
    return c >= m;
}

int main() {
    cin >> m >> n;
    t.resize(n);
    y.resize(n);
    z.resize(n);
    for (int i = 0; i < n; ++i) {
        cin >> t[i] >> z[i] >> y[i];
    }
    
    uint64_t low = 0, high = 1;
    while(!test(high)) high *= 2;
    while (low < high) {
        ull m = (low + high) / 2;
        if (test(m)) {
            high = m;
        }
        else {
            low = m + 1;
        }
    }

    cout << low << endl;

    // считаем, сколько за это время надуют всего
    ull c = 0;
    for (int i = 0; i < n; ++i) {
        c += count_balloons(i, low);
    }

    // c - лишние шарики
    c -= m;
    for (int i = 0; i < n; ++i) {
        ull k = count_balloons(i, low);
        // если i-й надул больше излишка, убирем у него
        // c шариков
        if (k >= c) {
            k -= c;
            c = 0;
        }
        else {
            k = 0;
            c -= k;
        }
        cout << k << ' ';
    }
    cout << endl;


}
