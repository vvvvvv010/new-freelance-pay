# NetLease Stellar 🚀
### Pay-as-you-go VPN & Cloud Access powered by Stellar (Soroban)

---

## 📌 1. Giới thiệu

**NetLease Stellar** là một dApp (Decentralized Application) được xây dựng trên nền tảng **Stellar Soroban**, cho phép người dùng thuê dịch vụ **VPN hoặc máy ảo (VM)** theo giờ với chi phí cực thấp và thanh toán tức thì bằng **USDC**.

Dự án hướng đến việc giải quyết bài toán thực tế tại Việt Nam:  
👉 Sinh viên và freelancer khó tiếp cận các dịch vụ quốc tế do:
- Không có thẻ tín dụng
- Chi phí thuê theo tháng cao
- Thiếu linh hoạt khi chỉ cần dùng trong thời gian ngắn

---

## 💡 2. Ý tưởng cốt lõi

Thay vì:
- Trả $5–$10/tháng cho VPN
- Hoặc thuê server dài hạn

👉 Người dùng chỉ cần:
- Trả **0.1 USDC (~2,500 VND)**  
- Sử dụng dịch vụ trong **1 giờ**
- Hết hạn → tự động ngắt quyền truy cập

⏱️ **Pay-as-you-go model (trả theo thời gian sử dụng)**

---

## 🎯 3. Mục tiêu dự án

- Xây dựng một hệ thống **micro-payment thực tế**
- Demonstrate khả năng của **Soroban smart contract**
- Tạo nền tảng cho mô hình **Cloud phi tập trung (Decentralized Cloud)**

---

## 🧱 4. Kiến trúc hệ thống

```text
User (Frontend)
   │
   ▼
Wallet (Freighter)
   │
   ▼
Soroban Smart Contract (NetLease)
   │
   ▼
Backend Server (VPN / VM Controller)
   │
   ▼
Infrastructure (VPN / Cloud VM)
