/**
 * Swift implementation
 */

struct VNode {
	let nodeType: Int // immutable
	let type: Any     // immutable, String or Function
	var props: [String: Any] // dictionary
	var children: [Any]
}

func Node (nodeType: Int, type: Any, props: [String: String], children: [VNode]) -> VNode {
	return VNode(nodeType: nodeType, type: type, props: props, children: children);
}

// an EmptyNode
var EmptyNode = Node(0, "", (), [])

// diff and determine the least amount of actions to update the view
func reconciler (newNode: VNode, oldNode: VNode) -> Int {
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
		// extractNode will handle when newNode.type is a component constructor instead of a string
		var currentNode: VNode = extractNode(newNode)

		// identical
		if currentNode == oldNode {
			return 0
		}

		// if not TextNode patch props
		if oldNode.nodeType == 1 { 
			patchProps(currentNode, oldNode)
		}

		var newLength: Int = currentNode.children.count
		var oldLength: Int = oldNode.children.count

		// remove all children
		if newLength == 0 {
			// but only if old children is not already cleared
			if oldLength != 0 {
				clearChildren(oldNode)
				oldNode.children = currentNode.children
			}
		} else {
			var deleteCount:Int = 0

			for var i:Int = 0; i < newLength || i < oldLength; i = i + 1 {
			    var newChild: VNode = currentNode.children[i] || EmptyNode
			    var oldChild: VNode = oldNode.children[i] || EmptyNode
			    var action: Int = reconciler(newChild, oldChild)

			    if action != 0 {
			    	var index:Int = i - deleteCount;

			    	switch action {
			    		// remove operation
			    		case 1: {
			    			removeNode(oldNode, index);
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
			    			replaceNode(newChild, oldChild)
			    			oldNode.children[index] = newChild
			    		}
			    		// key operation
			    		case 5: {
			    			// code block
			    		}
			    	}
			    }
			}
		}
	}

	return 0
}

// patch props
func patchProps (newNode: VNode, oldNode: VNode) {
	var diff: [Any] = diffProps(newNode.props, oldNode.props)
	var length: Int = diff.count

	if length != 0 {
		for var i = 0; i < length; i = i + 1 {
			var prop = diff[i]

			// patchProp makes calls to native api's
			patchProp(prop[0], prop[1], prop[2])
		}

		oldNode.props = newNode.props
	}
}

// diff props
func diffProps (newProps: [String: String], oldProps: [String: String]) -> [Any] {
	var diff: [Any] = []

	for (newName, newValue) in newProps { 
	    diff += diffNewProps(newProps, oldProps, newName, newValue)
	}

	for (oldName, oldValue) in oldProps {
	    diff += diffNewProps(newProps, oldProps, oldName, oldValue)
	}

	return diff
}

// diff new props
func diffNewProps (newProps: [String: String], oldProps: [String: String], newName: String, newValue: String) -> [Any] {
	var oldValue: String = oldProps[newName]
	var diff: [Any] = []

	if (newValue != nil && oldValue !== newValue) {
		diff += ["setAttribute", newName, newValue]
	}

	return diff
}

// diff old props
func diffOldProps (newProps: [String: String], oldProps: [String: String], oldName: String, oldValue: String) -> [Any] {
	var diff: [Any] = []

	if (newProps[oldName] == nil || newProps[oldName] === nil) {
		diff += ["removeAttribute", newName, newValue]
	}

	return diff
}


// a TextNode
var TextNode = Node(3, "Text", [:], ["Hello World"])
// an ElementNode NavBar with one TextNode child
var ElementNode = Node(1, "NavBar", ["state": "active"], [TextNode])

reconciler(ElementNode, ElementNode)