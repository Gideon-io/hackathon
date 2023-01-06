
fn main() {
    let target: u32 = 2;
    let nums = [1,4,5,8,9];
    let mut low: usize = 0;
    let mut high: usize = nums.len();
    let mut mid: usize;
    
    while low <= high{
        mid = low + (high -low)/2;
        
        if target == nums[mid]{
            print!("Target of {} has been found in the array", target);
            break;
        }
        else if target < nums[mid]{
            high = mid - 1;
        }
        else {
            low = mid + 1;
        }
    }
    
    
}
