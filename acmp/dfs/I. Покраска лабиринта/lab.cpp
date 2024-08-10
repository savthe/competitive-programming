#include <iostream>
#include <vector>

using namespace std;

int n;
vector<vector<char>> lab;

// dfs возвращает количество клеток, у которых соседняя клетка содержит
// # или если данная клетка на границе.

int dfs(int y, int x) {
    // если клекти (y, x) не существует, то выходим
    if (y < 0 || x < 0 || y >= n || x >= n || lab[y][x] != '.') {
        return 0;
    }

    // отмечаем клетку как посещенную
    lab[y][x] = '1';

    // считаем количество соседних клеток, в которых #.
    // условиями y == 0, x == 0, y == n-1, x == n-1 учтем 
    // граничные клетки
    int c = 0;
    c += (y == 0) || lab[y - 1][x] == '#';
    c += (x == 0) || lab[y][x - 1] == '#';
    c += (y == n - 1) || lab[y + 1][x] == '#';
    c += (x == n - 1) || lab[y][x + 1] == '#';

    // идем в 4 соседние клетки
    c += dfs(y - 1, x);
    c += dfs(y + 1, x);
    c += dfs(y, x - 1);
    c += dfs(y, x + 1);

    return c;
}

int main() {
    cin >> n;
    lab.resize(n, vector<char>(n));
    for (int i = 0; i < n; ++i) {
        for (int j = 0; j < n; ++j) {
            cin >> lab[i][j];
        }
    }

    // dfs из начальной клетки и из конечной.
    // вычитаем 4, так как у этих клеток нет 2-х стенок
    cout << 25 * (dfs(0, 0) + dfs(n - 1, n - 1) - 4) << endl;
}

