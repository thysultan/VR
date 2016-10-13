/**
 * Rust implementation
 */

use std::any::Any;
use std::collections::HashMap;

struct VNode {
    nodeType: i32,
    type: <Any>,
    props: HashMap<&str, <Any>>,
    children: Vec<Any>
}

// type is a reserved word in rust, thus the _type
fn Node (nodeType: i32, _type: <Any>, props: HashMap<&str, <Any>>, children: Vec<VNode>) -> VNode {
	return VNode(nodeType, _type, props, children);
}

// an emptyNode
let emptyNode: VNode = Node(0, "", HashMap::new(), [i32; 0]);

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
	else if (newNode.type !== oldNode.type) {
		return 4;
	}
	// recursive
	else {
		// extractNode will handle when newNode.type is a component constructor instead of a string
		let currentNode: VNode = extractNode(newNode);

		// identical, exit early
		if currentNode == oldNode {
			return 0;
		}

		// if not text patch props
		if oldNode.nodeType == 1 {
			patchProps(currentNode, oldNode);
		}

		let mut currentChildren = &currentNode.children[..];
		let mut newChildren = &currentNode.children[..];

		let mut newLength: i32 = currentChildren.len();
		let mut oldLength: i32 = oldChildren.len();

		if newLength == 0 {
			// but only if old children is not already cleared
			if oldLength != 0 {
				// clearChildren calls native api(s)
				clearChildren(oldNode);
				oldNode.children = currentNode.children;
			}
		} else {
			// on remove and creation(add) actions
			// we mutate the oldChildren's array
			// we store the delete/add count in this variable
			// to make sure we always have the right index on the next operation
			let mut deleteCount: i32 = 0;

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
			    	newChild = currentChildren[i];
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
			    	    	oldChildren.remove(index);
			    	    	deleteCount = deleteCount + 1;
			    	    }
			    	    // add operation
			    	    2 => {
			    	    	// addNode calls native api(s)
			    	    	addNode(oldNode, newChild, index);
			    	    	oldChildren.insert(index, newChild);
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
			    	    	replaceNode(newChild, oldChild);
			    	    	oldChild.children[index] = newChild;
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

// patch props
fn patchProps (newNode: <VNode>, oldNode: <VNode>) -> () {
	let diff: Vec<Any> = diffProps(newNode.get("props"), oldNode.get("props"));
	let length: i32 = diff.len();

	if length != 0 {

		// initialization
		let mut i: i32 = 0;

		loop {
			// increment
		    i += 1;
		    // condition
		    if i < length { break; }

		    let prop = diff[i];

			// patchProp calls native api(s)
			patchProp(oldNode, prop[0], prop[1], prop[2], prop[3]);
		}

		oldNode.props = newNode.props;
	}
}

// diff props
fn diffProps (newProps: HashMap<&str, <Any>>, oldProps: HashMap<&str, <Any>>) -> Vec<Any> {
	let mut diff: Vec<Any> = vec![];
	let NS: &str;

	match oldProps.get("xmlns") {
	    Some(value) => { NS = value; }
	    None => { NS = (); }
	}

	for (newName, newValue) in newProps.iter() {
		diff.append(&mut diffNewProps(newProps, oldProps, newName, newValue, NS));
	}

	for (oldName, oldValue) in oldProps.iter() {
		diff.append(&mut diffoldProps(newProps, oldProps, oldName, oldValue, NS));
	}

	return diff;
}

// diff new props
func diffNewProps (newProps: HashMap<&str, <Any>>, oldProps: HashMap<&str, <Any>>, newName: &str, newValue: <Any>, NS: &str) -> Vec<Any> {
	let mut diff: Vec<Any> = vec![];
	let mut oldValue: <Any>;

	// if the newProp's key is in oldProps assign oldValue to it
	match oldProps.get(newName) {
	    Some(value) => { oldValue = value; }
	    None => { oldValue = (); }
	}

	if newValue != nil && oldValue !== newValue) {
		diff.append(["setAttribute", newName, newValue, NS]);
	}

	return diff
}

// diff old props
fn diffOldProps (newProps: HashMap<&str, <Any>>, oldProps: HashMap<&str, <Any>>, oldName: &str, oldValue: <Any>, NS: &str) -> Vec<Any> {
	let mut diff: Vec<Any> = vec![];

	match newProps.get(oldName) {
	    Node => {
	    	diff.append(["removeAttribute", oldName, oldValue, NS]);
	    }
	}

	return diff;
}


// a text node
let mut textNode: VNode = Node(3, "Text", HashMap::new(), ["Hello World"]);

let props: HashMap<&str, <Any>> = HashMap::new();
props.insert("state", "active");

// an element node NavBar with one single child TextNode
let mut oldNode: VNode = Node(1, "NavBar", props, [textNode]);
let mut newNode: VNode = Node(1, "NavBar", props, [textNode]);

reconciler(newNode, oldNode);
