/**
 * Go implementation
 */

type VNode struct {
	nodeType int
	type: interface{}             // <Any> any type
	props: map[string]interface{} // dictionary
	children: []interface{}       // mmutable
}

// type is a reserved word in golang, thus the _type
func Node (nodeType int, _type interface{}, props map[string]interface{}, children: []interface{}) VNode {
	return VNode{nodeType, _type, props, children}
}

// an emptyNode
var emptyNode VNode = Node(0, "", map[string]interface{}{}, []interface{}{})

func reconciler (newNode VNode, oldNode VNode) int {
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

		// identical, exit early
		if currentNode == oldNode {
			return 0
		}

		// if not text patch props
		if oldNode.nodeType == 1 {
			patchProps(currentNode, oldNode)
		}

		var currentChildren []Vnode = currentNode.children[:]
		var oldChildren []Vnode = oldName.children[:]

		var newLength int = len(currentChildren)
		var oldLength int = len(oldChildren)

		// remove all children
		if newLength == 0 {
			// but only if old children is not already cleared
			if oldLength != 0 {
				// clearChildren calls native api(s)
				clearChildren(oldNode)
				oldNode.children = currentNode.children
			}
		} else {
			// on remove and creation(add) actions
			// we mutate the oldChildren's array
			// we store the delete/add count in this variable
			// to make sure we always have the right index on the next operation
			var deleteCount int = 0

			for var i int = 0; i < newLength || i < oldLength; i = i + 1 {
			    var newChild VNode = newLength >= i ? currentChildren[i] : emptyNode
			    var oldChild VNode = oldLength >= i ? oldChildren[i] : emptyNode

			    var action int = reconciler(newChild, oldChild)

			    if action != 0 {
			    	// we use this to resolve to the correct index 
			    	// because the index/length of the children array
			    	// could change over time in case of remove/creation actions
			    	var index int = i - deleteCount

			    	switch action {
			    		// remove operation
			    		case 1: {
			    			// removeNode calls native api(s)
			    			removeNode(oldNode, index)
			    			
							copy(oldChildren[index:], oldChildren[index+1:])
							oldChildren[len(oldChildren)-1] = nil
							var children = oldChildren[:len(oldChildren)-1]
							oldNode.children = children

			    			// update deleteCount, increment
			    			deleteCount = deleteCount + 1
			    		}
			    		// add operation
			    		case 2: {
			    			// addNode calls native api(s)
			    			addNode(oldNode, newChild, index)

			    			// insert
			    			var children = append(oldchildren, emptyNode)
			    			copy(children[index+1:], children[index:])
			    			children[index] = newChild
			    			oldNode.children = children

			    			// update deleteCount, decrement
			    			deleteCount = deleteCount - 1
			    		}
			    		// text operation
			    		case 3: {
			    			// updateText calls native api(s)
			    			updateText(newChild, oldChild)
			    			oldChild.children[0] = newChild.children[0]
			    		}
			    		// replace operation
			    		case 4: {
			    			// replaceNode calls native api(s)
			    			replaceNode(newChild, oldChild)
			    			oldchildren[index] = newChild
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
func patchProps (newNode VNode, oldNode VNode) {
	var diff []interface{} = diffProps(newNode.props, oldNode.props)
	var length int = len(diff)

	if length != 0 {
		for var i int = 0; i < length; i = i + 1 {
			var prop = diff[i]

			// patchProp calls native api(s)
			patchProp(oldNode, prop[0], prop[1], prop[2], prop[3])
		}

		oldNode.props = newNode.props
	}
}

// diff props
func diffProps (newProps map[string]interface{}, oldProps map[string]interface{}) []interface{} {
	var diff []interface = {}
	var NS string = oldProps["xmlns"]

	for newName, newValue := range newProps {
		diff = append(diff, diffNewProps(newProps, oldProps, newName, newValue, NS)...)
	}

	for oldName, oldValue := range oldProps {
	    diff = append(diff, diffoldProps(newProps, oldProps, oldName, oldValue, NS)...)
	}

	return diff
}

// diff new props
func diffNewProps (newProps map[string]interface{}, oldProps map[string]interface{}, newName string, newValue interface{}, NS string) []interface {
	var diff []interface{} = {}
	var oldValue interface{} = oldProps[newName]

	if newValue != nil && oldValue !== newValue) {
		diff += ["setAttribute", newName, newValue, NS]
		diff = append(diff, []interface{}{"setAttribute", newName, newValue, NS}...)
	}

	return diff
}

// diff old props
func diffOldProps (newProps map[string]interface{}, oldProps map[string]interface{}, oldName string, oldValue interface{}, NS string) []interface {
	var diff []interface{} = {}

	if newProps[oldName] == nil {
		diff = append(diff, []interface{}{"removeAttribute", oldName, oldValue, NS}...)
	}

	return diff
}


// a text node
var textNode Vnode = Node(3, "Text", map[string]interface{}{}, []interface{}{"Hello World"})
// an element node NavBar with one single child TextNode
var oldNode Vnode = Node(1, "NavBar", map[string]interface{}{"state": "active"}, []VNode{}{textNode})
var newNode Vnode = Node(1, "NavBar", map[string]interface{}{"state": "active"}, []VNode{}{textNode})

reconciler(newNode, oldNode)
