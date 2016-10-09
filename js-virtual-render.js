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
var EmptyNode = Node(0, "", (), []);

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

// a TextNode
var TextNode = Node(3, "Text", (), ["Hello World"])
// an ElementNode NavBar with one TextNode child
var ElementNode = Node(1, "NavBar", (state: "active"), [TextNode])

reconciler(ElementNode, ElementNode)