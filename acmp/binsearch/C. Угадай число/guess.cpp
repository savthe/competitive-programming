#include <iostream>
using namespace std;

int main() {
    int low = 1; 
    int high = 1e9;

    while(low <= high) {
        int m = (low + high) / 2;
        cout << m << endl;
        if (low == high) break;
        char rep; cin >> rep;
        if (rep == '=') break;
        if (rep == '<') high = m - 1;
        if (rep == '>') low = m + 1;
    }
}
