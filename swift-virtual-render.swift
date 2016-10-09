/**
 * Swift implementation
 */

struct Node {
	let nodeType: Int // immutable
	let type: Any     // immutable, String or Function
	var props: ()
	var children: [Any]
}

// a TextNode
var TextNode = Node(3, "Text", (), ["Hello World"])
// an ElementNode NavBar with one TextNode child
var ElementNode = Node(1, "NavBar", (state: "active"), [TextNode])
// an EmptyNode
var EmptyNode = Node(0, "", (), [])

func reconciler (newNode: Node, oldNode: Node) -> Int {
	// remove
	if newNode.nodeType == 0 {
		return 1
	}
	// add
	if oldNode.nodeType == 0 {
		return 2
	}
	// text
	if newNode.nodeType == 3 && oldNode.nodeType == 3 {
		// text update
		if newNode.children[0] !== oldNode.children[0] {
			return 3
		}
	}
	// keys
	else if newNode.props.key != oldNode.props.key {
		return 5
	}
	// replace
	else if newNode.type != oldNode.type {
		return 4
	}
	// recursive
	else {
		// identical
		if newNode == oldNode {
			return 0
		}

		// patch props, non TextNodes
		if oldNode.nodeType == 1 { 
			patchProps(newNode, oldNode)
		}

		var newLength: Int = newNode.children.count
		var oldLength: Int = oldNode.children.count

		// remove all children
		if newLength == 0 {
			// but only if old children is not already cleared
			if oldLength != 0 {
				clearChildren(oldNode)
				oldNode.children = newNode.children
			}
		} else {
			var deleteCount:Int = 0

			for var i:Int = 0; i < newLength || i < oldLength; i = i + 1 {
			    var newChild: Node = newNode.children[i] || Node(0, '', (), [])
			    var oldChild: Node = oldNode.children[i] || Node(0, '', (), [])
			    var action: Int    = reconciler(newChild, oldChild)

			    if action != 0 {
			    	var index:Int = i - deleteCount;

			    	switch action {
			    		// remove operation
			    		case 1: {
			    			removeChild(oldNode, index);
			    			oldNode.children.removeAtIndex(index)
			    			deleteCount = deleteCount + 1
			    		}
			    		// add operation
			    		case 2: {
			    			addNode(oldNode, newChild, index)
			    			oldNode.children.insert(newChild, atIndex: index)
			    			deleteCount = deleteCount - 1
			    		}
			    		// text operation
			    		case 3: {
			    			updateText(newChild, oldChild)
			    			oldChild.children[0] = newChild.children[0]
			    		}
			    		// replace operation
			    		case 4: {
			    			// replace dom node
			    			replaceChild(newChild, oldChild)
			    			oldNode.children[index] = newChild
			    		}
			    		// key operation
			    		case 5: {

			    		}
			    	}
			    }
			}
		}
	}

	return 0
}