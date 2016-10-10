/**
 * Rust implementation
 */

struct VNode {
    nodeType: i32,
    nodeName: <Any>,
    props: HashMap<&str, <Any>>,
    children: Vec<VNode>
}

fn Node (nodeType: i32, nodeName: <Any>, props: HashMap<&str, <Any>>, children: Vec<VNode>) -> VNode {
	return VNode(nodeType, type, props, children);
}

// an emptyNode
let emptyNode: VNode = Node(0, "", (::std::collections::HashMap::new()), [i32; 0]);

fn reconciler (newNode: <VNode>, oldNode: <VNode>) -> i32 {
	// remove
	if (newNode.nodeType == 0) {
		return 1;
	}
	// add
	if (oldNode.nodeType == 0) {
		return 2;
	}
	// text
	if (newNode.nodeType === 3 && oldNode.nodeType === 3) {
		// text update
		if (newNode.children[0] != oldNode.children[0]) {
			return 3;
		}
	}
	// keys
	else if (newNode.props.key !== oldNode.props.key) {
		return 5;
	}
	// replace
	else if (newNode.nodeName !== oldNode.nodeName) {
		return 4;
	}
	// recursive
	else {
		// extractNode will handle when newNode.nodeName is a component constructor instead of a string
		let currentNode: VNode = extractNode(newNode);

		// identical, exit early
		if currentNode == oldNode {
			return 0;
		}

		// if not text patch props
		if oldNode.nodeType == 1 {
			patchProps(currentNode, oldNode);
		}


		let mut newLength: i32 = currentNode.children.len()
		let mut oldLength: i32 = oldNode.children.len()

		if newLength == 0 {
			// but only if old children is not already cleared
			if oldLength != 0 {
				// clearChildren calls native api(s)
				clearChildren(oldNode);
				oldNode.children = currentNode.children;
			}
		} else {
			let mut deleteCount: i32 = 0l

			// classic c-lang like for-loop impl
			// initialization
			let mut i: i32 = 0;

			loop {
				// increment
			    i += 1;
			    // condition
			    if i < newLength || i < oldLength { break; }

			    let newChild: <VNode>;
			    let oldChild: <VNode>;

			    if newLength >= i { // currentNode's children has an element at that index
			    	newChild = currentNode.children[i];
			    } else { // element does not exist, assign an emptyNode
			    	newChild = emptyNode;
			    }

			    if oldLength >= i { // oldNode's children has an element at that index
			    	oldChild = oldChildren[i];
			    } else { // element does not exist, assign an emptyNode
			    	oldChild = emptyNode;
			    }

			    let action: i32 = reconciler(newChild, oldChild);

			    if action != 0 {
			    	let index: i32 = i - deleteCount;

			    	match action {
			    		// remove operation
			    	    1 => { 
			    	    	// removeNode calls native api(s)
			    	    	removeNode(oldNode, index);
			    	    	oldchildren.remove(index);
			    	    	deleteCount = deleteCount + 1;
			    	    }
			    	    // add operation
			    	    2 => {
			    	    	// addNode calls native api(s)
			    	    	addNode(oldNode, newChild, index);
			    	    	oldchildren.insert(index, newChild);
			    	    	deleteCount = deleteCount - 1;
			    	    },
			    	    // text operation
			    	    3 => {
			    	    	// updateText calls native api(s)
			    	    	updateText(newChild, oldChild);
			    	    	oldChild.children[0] = newChild.children[0];
			    	    },
			    	    // replace operation
			    	    4 => {
			    	    	// replaceNode calls native api(s)
			    	    	replaceNode(newChild, oldChild)
			    	    	oldchildren[index] = newChild
			    	    },
			    	    // key operation
			    	    5 => {
			    	    	// code block
			    	    }
			    	}
			    }
			}
		}
	}

	return 0;
}
