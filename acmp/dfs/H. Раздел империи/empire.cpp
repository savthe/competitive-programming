#include <iostream>
#include <vector>

using namespace std;

vector<vector<int>> adj;
vector<bool> capitals;
vector<int> colors;

void dfs(int v)
{
    for (auto nb: adj[v]) {
        if (colors[nb] == 0) {
            colors[nb] = colors[v];
            dfs(nb);
        }
    }
}

int main() 
{
    int n, m; cin >> n >> m;
    adj.resize(n + 1);
    capitals.resize(n + 1);
    colors.resize(n + 1);

    while (m--) {
        int a, b; cin >> a >> b;
        adj[a].push_back(b);
        adj[b].push_back(a);
    }

    int k; cin >> k;
    while (k--) {
        int a; cin >> a;
        capitals[a] = true;
        colors[a] = a;
    }

    for (int i = 1; i <= n; ++i) {
        if (capitals[i]) {
            dfs(i);
        }
    }

    for (int i = 1; i <= n; ++i) {
        if (capitals[i]) {
            vector<int> res;
            for (int j = 1; j <= n; ++j) {
                if (colors[j] == i) {
                    res.push_back(j);
                }
            }
            cout << res.size() << endl;
            for (auto v: res) {
                cout << v << ' ';
            }
            cout << endl;
        }
    }

}

