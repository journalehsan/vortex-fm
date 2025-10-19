use std::path::PathBuf;
use std::collections::HashSet;

/// Navigation history using double stack approach
/// - back_stack: History of visited directories
/// - forward_stack: Future directories (when going back)
/// - current: Current directory
/// - visited: Quick lookup for visited paths
#[derive(Debug, Clone)]
pub struct NavigationHistory {
    pub back_stack: Vec<PathBuf>,
    pub forward_stack: Vec<PathBuf>,
    pub current: PathBuf,
    pub visited: HashSet<PathBuf>,
}

impl NavigationHistory {
    pub fn new(initial: PathBuf) -> Self {
        let mut visited = HashSet::new();
        visited.insert(initial.clone());
        
        Self {
            back_stack: Vec::new(),
            forward_stack: Vec::new(),
            current: initial,
            visited,
        }
    }
    
    /// Navigate to a new directory
    pub fn navigate_to(&mut self, path: PathBuf) {
        // Don't navigate to the same directory
        if self.current == path {
            return;
        }
        
        // Move current to back stack
        self.back_stack.push(std::mem::take(&mut self.current));
        self.current = path.clone();
        self.visited.insert(path);
        
        // Clear forward stack on new navigation
        self.forward_stack.clear();
    }
    
    /// Go back to previous directory
    pub fn go_back(&mut self) -> Option<PathBuf> {
        if let Some(prev) = self.back_stack.pop() {
            self.forward_stack.push(std::mem::take(&mut self.current));
            self.current = prev;
            Some(self.current.clone())
        } else {
            None
        }
    }
    
    /// Go forward to next directory
    pub fn go_forward(&mut self) -> Option<PathBuf> {
        if let Some(next) = self.forward_stack.pop() {
            self.back_stack.push(std::mem::take(&mut self.current));
            self.current = next;
            Some(self.current.clone())
        } else {
            None
        }
    }
    
    /// Go up to parent directory
    pub fn go_up(&mut self) -> Option<PathBuf> {
        if let Some(parent) = self.current.parent() {
            let parent_path = parent.to_path_buf();
            self.navigate_to(parent_path);
            Some(self.current.clone())
        } else {
            None
        }
    }
    
    /// Check if we can go back
    pub fn can_go_back(&self) -> bool {
        !self.back_stack.is_empty()
    }
    
    /// Check if we can go forward
    pub fn can_go_forward(&self) -> bool {
        !self.forward_stack.is_empty()
    }
    
    /// Check if we can go up (has parent directory)
    pub fn can_go_up(&self) -> bool {
        self.current.parent().is_some()
    }
    
    /// Get current directory
    pub fn current(&self) -> &PathBuf {
        &self.current
    }
    
    /// Get history size (for debugging)
    pub fn history_size(&self) -> usize {
        self.back_stack.len()
    }
    
    /// Get forward size (for debugging)
    pub fn forward_size(&self) -> usize {
        self.forward_stack.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_navigation_history() {
        let home = PathBuf::from("/home/user");
        let mut history = NavigationHistory::new(home.clone());
        
        // Test initial state
        assert_eq!(history.current(), &home);
        assert!(!history.can_go_back());
        assert!(!history.can_go_forward());
        assert!(history.can_go_up());
        
        // Test navigation
        let docs = PathBuf::from("/home/user/Documents");
        history.navigate_to(docs.clone());
        assert_eq!(history.current(), &docs);
        assert!(history.can_go_back());
        assert!(!history.can_go_forward());
        
        // Test going back
        let back_path = history.go_back();
        assert_eq!(back_path, Some(home.clone()));
        assert_eq!(history.current(), &home);
        assert!(!history.can_go_back());
        assert!(history.can_go_forward());
        
        // Test going forward
        let forward_path = history.go_forward();
        assert_eq!(forward_path, Some(docs.clone()));
        assert_eq!(history.current(), &docs);
        assert!(history.can_go_back());
        assert!(!history.can_go_forward());
    }
    
    #[test]
    fn test_go_up() {
        let docs = PathBuf::from("/home/user/Documents");
        let mut history = NavigationHistory::new(docs);
        
        let up_path = history.go_up();
        assert_eq!(up_path, Some(PathBuf::from("/home/user")));
        assert_eq!(history.current(), &PathBuf::from("/home/user"));
    }
}
