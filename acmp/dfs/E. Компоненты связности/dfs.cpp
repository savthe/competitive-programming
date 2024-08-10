#include <iostream>
#include <vector>

using namespace std;

vector<vector<int>> adj;
vector<bool> used;
vector<vector<int>> comps;

void dfs(int v, int c) {
  used[v] = true;
	comps[c].push_back(v);
  for (auto nb: adj[v]) 
  if (!used[nb]) dfs(nb, c);
}

int  main() {
  int n, m; cin >> n >> m;

  adj.resize(n+1);
	used.resize(n+1);
	comps.resize(n+1);
	while(m--) {
    int a, b; cin >> a >> b;
    adj[a].push_back(b);
    adj[b].push_back(a);
  }

  int c = 0;
  for(int i = 1; i <= n; ++i) {
    // Если вершина i еще не посещена, то и вся компонента связности, содержащая ее,
    // тоже не посещена.
    if (!used[i]) {
      dfs(i, c);
      c++;
    }
  }

  cout << c << endl;
  for (int i = 0; i < c; ++i) {
    cout << comps[i].size() << endl;
    for (auto x : comps[i]) cout << x << ' ';
    cout << endl;
  }
}
