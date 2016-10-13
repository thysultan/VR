/**
 * C++ implementation
 */

#include <vector>
#include <string>
#include <unordered_map>

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

		// ...
	}

	return 0;
}
