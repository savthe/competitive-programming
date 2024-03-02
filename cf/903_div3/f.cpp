#include <iostream>
#include <vector>

using namespace std;

vector<vector<int>> tree;
vector<bool> used;

// Computes dist array, where dist[k] is a distance between initial vertex and vertex k.
void dfs(int v, int depth, vector<int>& dist) {
  dist[v] = depth;
  for (auto nb : tree[v]) {
    if (!used[nb]) {
       used[nb] = true;
       dfs(nb, depth + 1, dist);
    }
  }
}

void solve() {
  tree.clear();

  int n, k; cin >> n >> k;

  vector<int> marked(k);
  for (auto& x : marked) cin >> x;

  tree.resize(n + 1);
  for (int i = 0; i < n - 1; ++i) {
    int u, v; cin >> u >> v;
    tree[u].push_back(v);
    tree[v].push_back(u);
  }

  // Solution idea. We consider only a subtree with each leaf being a marked vertex.
  // In this tree we find a max distance between two marked leaves (a diameter).
  // The result is half of this distance rounding up.

  // A lambda which find farthest marked vertex from v and returns this vertex with found distance.
  auto max_dist = [&](int v) -> pair<int, int> {
    used.assign(n+1, false);
    used[v] = true;
    vector<int> dist(n + 1);
    dfs(v, 0, dist);

    // Now for each vertex in tree we know the distance from v.
    // Find farthest marked vertex and its distance.
    v = marked[0];
    for (auto t : marked) {
      if (dist[v] < dist[t]) v = t;
    }
    return {v, dist[v]};
  };

  // u is farthest from marked[0]
  auto [u, unused1] = max_dist(marked[0]);
  // v is farthest from u. Path from v to u is one of the longest paths in tree between two marked
  // vertices.
  auto [unused2, d] = max_dist(u);

  cout << (d + 1) / 2 << endl;
}

int main() {
  int t; cin >> t;
  while(t--) {
    solve();
  }
}
