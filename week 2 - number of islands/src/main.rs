// https://leetcode.com/problems/number-of-islands/

impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        fn dfs(grid: &mut Vec<Vec<char>>, i: usize, j:usize) {
            if (i < 0 || j < 0 || i >= grid.len() || j >= grid[0].len() || grid[i][j] != '1'){
                return;
            }
            
            grid[i][j] = '#';
            
            dfs(grid, i+1, j);
            dfs(grid, i-1, j);
            dfs(grid, i, j+1);
            dfs(grid, i, j-1);
        } 
        
        let mut islandCount = 0;
        
        for i in (0..grid.len()) {
            for j in (0..grid[0].len()) {
                if(grid[i][j] == '1'){
                    islandCount += 1;
                    dfs(&mut grid, i, j);
                }
            }
        }
        
        return islandCount;   
    } 
}
