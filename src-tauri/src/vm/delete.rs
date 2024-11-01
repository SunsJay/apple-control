use crate::vm::vmrun;

// pub async fn vmrun_copy_file_from_guest_to_host(index: usize, vm_name: String, guest_vmx: String) {
//     println!("  > 线程  {}", index);
//     println!("  > {} ==> 检查ID", vm_name);
//     println!();
//     let vm_path = format!("D:\\vm-sms\\{}.txt", vm_name);
//     let guest_path = "/Users/cc/apple-id.txt".to_string();
//     let vm_path_clone = vm_path.clone();
//     let guest_vmx_clone = guest_vmx.clone();
//     let guest_path_clone = guest_path.clone();
//
//     let _ =
//         vmrun(vec![
//             "-T".to_string(),
//             "ws".to_string(),
//             "-gu".to_string(),
//             "cc".to_string(),
//             "-gp".to_string(),
//             "123456".to_string(),
//             "copyFileFromGuestToHost".to_string(),
//             guest_vmx_clone,
//             guest_path_clone,
//             vm_path_clone,
//         ]),
//
//
//
//     let content = read(&vm_path).unwrap_or_default();
//     // if !content.is_empty() {
//     //     apple_id_upload(index, &vm_name, content.clone())
//     //         .await
//     //         .unwrap();
//     // }
//
//     let guest_vmx_clone = guest_vmx.clone();
//     let guest_path_clone = guest_path.clone();
//     let _ =
//         vmrun(vec![
//             "-T".to_string(),
//             "ws".to_string(),
//             "-gu".to_string(),
//             "cc".to_string(),
//             "-gp".to_string(),
//             "123456".to_string(),
//             "deleteFileInGuest".to_string(),
//             guest_vmx_clone,
//             guest_path_clone,
//         ],
//     )
//         .await;
// }