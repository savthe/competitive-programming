// WA 
#include <iostream>
#include <cassert>

using namespace std;

using ll = uint64_t;
int main() {
    ll n = 1;
    ll r = 0;
    ll rep;
    for (;;) {
        cout << "? " << n << endl;
        cin >> rep;
        //assert(rep == 0 || rep != r);
        if (rep == 0 || rep < r) break;
        r = rep;
        n *= 2;
    }
    n -= rep;
    cout << "! " << n << endl;
}

