pub use crate::vp8::common::types::vp8_prob;
pub type vp8_tree_index = ::core::ffi::c_schar;
#[derive(Copy, Clone, Default)]
#[repr(C)]
pub struct vp8_token_struct {
    pub value: ::core::ffi::c_int,
    pub Len: ::core::ffi::c_int,
}
pub type vp8_token = vp8_token_struct;
pub type uint64_t = u64;
pub const vp8_prob_half: vp8_prob = 128 as ::core::ffi::c_int as vp8_prob;

// Safe recursive helper to traverse the tree and populate the token table.
fn tree2tok_safe(
    p: &mut [vp8_token],
    t: &[vp8_tree_index],
    mut i: i32,
    mut v: i32,
    mut L: i32,
    offset: i32,
) {
    v += v;
    L += 1;
    loop {
        let fresh0 = i;
        i = i + 1;
        let j = t[fresh0 as usize] as i32;
        if j <= 0 {
            let idx = -j - offset;
            p[idx as usize].value = v;
            p[idx as usize].Len = L;
        } else {
            tree2tok_safe(p, t, j, v, L, offset);
        }
        v += 1;
        if (v & 1) == 0 {
            break;
        }
    }
}

// Safe helper to pre-scan tree bounds.
unsafe fn get_tree_bounds(
    t: *const vp8_tree_index,
    i: i32,
    visited: &mut [bool; 256],
) -> (i32, i32) {
    if i < 0 || i >= 256 || visited[i as usize] {
        return (0, 0);
    }
    visited[i as usize] = true;
    visited[(i + 1) as usize] = true;
    
    let mut max_token = 0;
    let mut max_tree = i + 1;
    
    let j1 = *t.offset(i as isize) as i32;
    if j1 <= 0 {
        max_token = std::cmp::max(max_token, -j1);
    } else {
        let (tok, tr) = get_tree_bounds(t, j1, visited);
        max_token = std::cmp::max(max_token, tok);
        max_tree = std::cmp::max(max_tree, tr);
    }
    
    let j2 = *t.offset((i + 1) as isize) as i32;
    if j2 <= 0 {
        max_token = std::cmp::max(max_token, -j2);
    } else {
        let (tok, tr) = get_tree_bounds(t, j2, visited);
        max_token = std::cmp::max(max_token, tok);
        max_tree = std::cmp::max(max_tree, tr);
    }
    
    (max_token, max_tree)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_tokens_from_tree(
    p: *mut vp8_token_struct,
    t: *const vp8_tree_index,
) {
    if p.is_null() || t.is_null() {
        return;
    }
    unsafe {
        let mut visited = [false; 256];
        let (max_token, max_tree) = get_tree_bounds(t, 0, &mut visited);
        let p_slice = core::slice::from_raw_parts_mut(p, (max_token + 1) as usize);
        let t_slice = core::slice::from_raw_parts(t, (max_tree + 1) as usize);
        tree2tok_safe(p_slice, t_slice, 0, 0, 0, 0);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_tokens_from_tree_offset(
    p: *mut vp8_token_struct,
    t: *const vp8_tree_index,
    offset: ::core::ffi::c_int,
) {
    if p.is_null() || t.is_null() {
        return;
    }
    unsafe {
        let mut visited = [false; 256];
        let (max_token, max_tree) = get_tree_bounds(t, 0, &mut visited);
        let limit = (max_token - offset + 1) as usize;
        let p_slice = core::slice::from_raw_parts_mut(p, limit);
        let t_slice = core::slice::from_raw_parts(t, (max_tree + 1) as usize);
        tree2tok_safe(p_slice, t_slice, 0, 0, 0, offset);
    }
}

fn branch_counts_safe(
    n: i32,
    tok: &[vp8_token],
    tree: &[vp8_tree_index],
    branch_ct: &mut [[u32; 2]],
    num_events: &[u32],
) {
    let tree_len = n - 1;
    for t in 0..tree_len as usize {
        branch_ct[t][0] = 0;
        branch_ct[t][1] = 0;
    }
    for t in 0..n as usize {
        let mut L = tok[t].Len;
        let enc = tok[t].value;
        let ct = num_events[t];
        let mut i = 0 as vp8_tree_index;
        loop {
            L -= 1;
            let b = (enc >> L) & 1;
            let j = (i as i32) >> 1;
            branch_ct[j as usize][b as usize] = branch_ct[j as usize][b as usize].wrapping_add(ct);
            i = tree[(i as i32 + b) as usize];
            if i <= 0 {
                break;
            }
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vp8_tree_probs_from_distribution(
    n: ::core::ffi::c_int,
    tok: *const vp8_token,
    tree: *const vp8_tree_index,
    probs: *mut vp8_prob,
    branch_ct: *mut [::core::ffi::c_uint; 2],
    num_events: *const ::core::ffi::c_uint,
    Pfactor: ::core::ffi::c_uint,
    Round: ::core::ffi::c_int,
) {
    if tok.is_null() || tree.is_null() || probs.is_null() || branch_ct.is_null() || num_events.is_null() {
        return;
    }
    unsafe {
        let tok_slice = core::slice::from_raw_parts(tok, n as usize);
        let tree_slice = core::slice::from_raw_parts(tree, (2 * (n - 1)) as usize);
        let probs_slice = core::slice::from_raw_parts_mut(probs, (n - 1) as usize);
        let branch_ct_slice = core::slice::from_raw_parts_mut(branch_ct as *mut [u32; 2], (n - 1) as usize);
        let num_events_slice = core::slice::from_raw_parts(num_events, n as usize);
        
        branch_counts_safe(n, tok_slice, tree_slice, branch_ct_slice, num_events_slice);
        
        let tree_len = n - 1;
        for t in 0..tree_len as usize {
            let c = &branch_ct_slice[t];
            let tot = c[0].wrapping_add(c[1]);
            if tot != 0 {
                let p = ((c[0] as u64)
                    .wrapping_mul(Pfactor as u64)
                    .wrapping_add(
                        (if Round != 0 {
                            tot >> 1
                        } else {
                            0
                        }) as u64,
                    ) as u32)
                    .wrapping_div(tot);
                probs_slice[t] = (if p < 256 {
                    if p != 0 {
                        p
                    } else {
                        1
                    }
                } else {
                    255
                }) as vp8_prob;
            } else {
                probs_slice[t] = vp8_prob_half;
            }
        }
    }
}
