/**
 * C++ implementation
 */

#include <vector>
#include <string>
#include <unordered_map>

// so we can do unordered_map instead of std::unordered_map
using namespace std;

template <typename Any>

struct VNode {
	int nodeType;
	Any type;
	unordered_map<string, Any> props;
	vector<Any> children;
};

int reconciler (VNode newNode, VNode oldNode) {
	if (newNode.nodeType == 0) {
		return 1;
	}
	// add
	if (oldNode.nodeType == 0) {
		return 2;
	}
	// text
	if (newNode.nodeType == 3 && oldNode.nodeType == 3) {
		// text update
		if (newNode.children[0] !== oldNode.children[0]) {
			return 3;
		}
	}
	// keys
	else if (newNode.props.key != oldNode.props.key) {
		return 5;
	}
	// replace
	else if (newNode.type != oldNode.type) {
		return 4;
	}
	// recursive
	else {
		// extractNode will handle when newNode.type is a component constructor instead of a string
		VNode currentNode = extractNode(newNode);

		// identical, exit early
		if (currentNode == oldNode) {
			return 0;
		}

		// if not text patch props
		if (oldNode.nodeType == 1) {
			patchProps(currentNode, oldNode);
		}

		vector<VNode> currentChildren = currentNode.children;
		vector<VNode> oldChildren = oldName.children;

		int newLength = currentChildren.size();
		int oldLength = oldChildren.size();

		// remove all children
		if (newLength == 0) {
			// but only if old children is not already cleared
			if (oldLength != 0) {
				// clearChildren calls native api(s)
				clearChildren(oldNode);
				oldNode.children = currentChildren;
			}
		} else {
			// on remove and creation(add) actions
			// we mutate the oldChildren's array
			// we store the delete/add count in this variable
			// to make sure we always have the right index on the next operation
			int deleteCount = 0;

			for (int i = 0; i < newLength || i < oldLength; i = i + 1) {
			    VNode newChild = newLength >= i ? currentChildren[i] : emptyNode;
			    VNode oldChild = oldLength >= i ? oldChildren[i] : emptyNode;

			    int action = reconciler(newChild, oldChild);

			    if (action != 0) {
			    	// we use this to resolve to the correct index 
			    	// because the index/length of the children array
			    	// could change over time in case of remove/creation actions
			    	int index = i - deleteCount;

			    	switch (action) {
			    		// remove operation
			    		case 1: {
			    			
			    		}
			    		// add operation
			    		case 2: {
			    			
			    		}
			    		// text operation
			    		case 3: {
			    			
			    		}
			    		// replace operation
			    		case 4: {
			    			
			    		}
			    		// key operation
			    		case 5: {
			    			
			    		}
			    	}
			    }
			}
		}
	}

	return 0;
}


void patchProps (VNode newNode, VNode oldNode) {
	vector<Any> diff = diffProps(newNode.props, oldNode.props)
	int length = diff.size()

	if (length != 0) {
		for (int i = 0; i < length; i = i + 1) {
			vector<Any> prop = diff[i];

			// patchProp calls native api(s)
			patchProp(oldNode, prop[0], prop[1], prop[2], prop[3])
		}

		oldNode.props = newNode.props;
	}
}

vector<Any> diffProps (unordered_map<string, Any> newProps, unordered_map<string, Any> oldProps) {
	vector<Any> diff = {};

	return diff;
}

vector<Any> diffNewProps (unordered_map<string, Any> newProps, unordered_map<string, Any> oldProps, string newName, Any newValue, string NS) {
	vector<Any> diff = {};

	return diff;
}

vector<Any> diffOldProps (unordered_map<string, Any> newProps, unordered_map<string, Any> oldProps, string newName, Any newValue, string NS) {
	vector<Any> diff = {};

	return diff;
}
