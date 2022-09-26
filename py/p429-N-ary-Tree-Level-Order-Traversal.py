"""
# Definition for a Node.
class Node:
    def __init__(self, val=None, children=None):
        self.val = val
        self.children = children
"""




class Solution:
    def levelOrder(self, root: 'Node') -> list[list[int]]:
        if root is None:
            return []

        ret: list[list[int]] = []
        stack = [root]

        while len(stack) > 0:
            level_vals = []
            next_level_nodes = []

            for node in stack:
                level_vals.append(node.val)
                next_level_nodes.extend(node.children)
            ret.append(level_vals)
            stack = next_level_nodes
        return ret


class Solution2:
    def levelOrder(self, root: 'Node') -> list[list[int]]:
        if root is None:
            return []

        from queue import Queue
        q = Queue()
        q.put_nowait(root)
        ret = []
        while not q.empty():
            vals = []
            count = q.qsize()
            while count > 0:
                count -= 1
                node = q.get_nowait()
                vals.append(node.val)
                for c in node.children:
                    q.put_nowait(c)
            ret.append(vals)
        return ret
            

# Definition for a Node.
class Node:
    def __init__(self, val=None, children=None):
        self.val = val
        self.children = children
