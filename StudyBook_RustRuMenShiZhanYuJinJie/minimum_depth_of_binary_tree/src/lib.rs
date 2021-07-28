/*
根节点到叶子节点的最小深度可使用深度优先搜索算法以递归的方式来计算。其分为以下3种情况。
1）当左、右子树都为空时，只有1个根节点，深度为1（根节点与叶子节点重合）。
2）当左、右子树有一个为空时，返回的是非空子树的最小深度，而不是空子树的深度0。
   若返回0相当于把当前节点视成叶子节点，与此节点有非空子树矛盾。
3）当左、右子树都不为空时，返回左、右子树深度的较小值
*/
pub mod solution1;
