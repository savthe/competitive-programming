#include <iostream>
using namespace std;

int main() {
    int low_x = -1e9, high_x = 1e9;
    int low_y = -1e9, high_y = 1e9;

    for(;;) {
        int m_x = (low_x + high_x) / 2;
        int m_y = (low_y + high_y) / 2;
        cout << m_y << ' ' << m_x << endl;

        int dx, dy;
        cin >> dy >> dx;

        if(dx == 0 && dy == 0) break;

        if (dx == -1) { high_x = m_x - 2; low_x--; }
        if (dx == 1) { low_x = m_x + 2; high_x++; }
        if (dy == -1) { high_y = m_y - 2; low_y--; }
        if (dy == 1) { low_y = m_y + 2; high_y++; }
    }
}

