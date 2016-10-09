/**
 * Javascript implementation
 */

class VNode () {
	constructor (nodeType, type, props, children) {
		this.nodeType = nodeType;
		this.type = type;
		this.props = props;
		this.children = children;
	}
}

function Node (nodeType, type, props, children) {
	return new VNode(nodeType, type, props, children);
}

// an EmptyNode
var EmptyNode = Node(0, "", {}, []);

function reconciler (newNode, oldNode) {
	// remove
	if (newNode.nodeType === 0) {
		return 1;
	}
	// add
	if (oldNode.nodeType === 0) {
		return 2;
	}
	// text
	if (newNode.nodeType === 3 && oldNode.nodeType === 3) {
		// text update
		if (newNode.children[0] !== oldNode.children[0]) {
			return 3;
		}
	}
	// keys
	else if (newNode.props.key !== oldNode.props.key) {
		return 5;
	}
	// replace
	else if (newNode.type !== oldNode.type) {
		return 4;
	}
	// recursive
	else {
		// extract will handle when newNode.type is a component constructor instead of a string
		var currentNode = extractNode(newNode)

		// identical
		if (currentNode === oldNode) {
			return 0;
		}

		// if not TextNode patch props
		if (oldNode.nodeType === 1) { 
			patchProps(currentNode, oldNode);
		}

		var newLength = currentNode.children.length;
		var oldLength = oldNode.children.length;

		// remove all children
		if (newLength === 0) {
			// but only if old children is not already cleared
			if (oldLength !== 0) {
				clearChildren(oldNode);
				oldNode.children = currentNode.children;
			}
		} else {
			var deleteCount = 0;

			for (var i = 0; i < newLength || i < oldLength; i = i + 1) {
			    var newChild = currentNode.children[i] || EmptyNode;
			    var oldChild = oldNode.children[i] || EmptyNode;
			    var action   = reconciler(newChild, oldChild);

			    if (action !== 0) {
			    	var index = i - deleteCount;

			    	switch (action) {
			    		// remove operation
			    		case 1: {
			    			removeNode(oldNode, index);
			    			oldNode.children.splice(index, 1);
			    			deleteCount = deleteCount + 1;
			    			break;
			    		}
			    		// add operation
			    		case 2: {
			    			addNode(oldNode, newChild, index);
			    			oldNode.children.splice(index, 0, newChild);
			    			deleteCount = deleteCount - 1;
			    			break;
			    		}
			    		// text operation
			    		case 3: {
			    			updateText(newChild, oldChild);
			    			oldChild.children[0] = newChild.children[0];
			    			break;
			    		}
			    		// replace operation
			    		case 4: {
			    			// replace dom node
			    			replaceNode(newChild, oldChild);
			    			oldNode.children[index] = newChild;
			    			break;
			    		}
			    		// key operation
			    		case 5: {
			    			// code block
			    			break;
			    		}
			    	}
			    }
			}
		}
	}

	return 0;
}

// patch props
function patchProps (newNode, oldNode) {
	var diff = diffProps(newNode.props, oldNode.props);
	var length = diff.length;

	if (length !== 0) {
		for (var i = 0; i < length; i = i + 1) {
			var prop = diff[i];

			// patchProp makes calls to native api's
			patchProp(prop[0], prop[1], prop[2], prop[3])
		}

		oldNode.props = newNode.props
	}
}

// diff props
function diffProps (newProps, oldProps) {
	var diff = [];

	for (var newName in newProps) { 
		diff[diff.length] = diffNewProps(newProps, oldProps, newName, NS, diff); 
	}
	for (var oldName in oldProps) { 
		diff[diff.length] = diffOldProps(newProps, oldName, oldValue, NS, diff); 
	}

	return diff;
}

// diff new props
function diffNewProps (newProps, oldProps, newName, newValue, NS, diff) {
	var newValue = newProps[newName]; 
	var oldValue = oldProps[newName];

	if (newValue !== void 0 && newValue !== null && oldValue !== newValue) {
		diff[diff.length] = ['setAttribute', newName, newValue, NS];
	}
}

// diff old props
function diffOldProps (newProps, oldProps, oldName, oldValue, NS, diff) {
	if (newProps[oldName] === null || newProps[oldName] === void 0) {
		diff[diff.length] = ['removeAttribute', oldName, oldValue, NS];
	}
}

// a TextNode
var TextNode = Node(3, "Text", {}, ["Hello World"])
// an ElementNode NavBar with one TextNode child
var ElementNode = Node(1, "NavBar", {state: "active"}, [TextNode])

reconciler(ElementNode, ElementNode)