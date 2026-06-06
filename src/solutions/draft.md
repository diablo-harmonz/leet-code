Bài 1: Cộng hai số
1. Tạo một cuốn sổ cái rỗng
2. Duyệt mảng từ tham số được truyền vào
    3. Lấy tổng của target - giá trị số hiện tại đang ở vị trí được duyệt
    4. Nếu tổng đó tồn tại trong cuốn sổ cái, trả về chỉ số của nó
    5. Nếu không tồn tại, thêm giá trị hiện tại và chỉ số của nó vào cuốn sổ cái

----
Bài 2: Xác minh số đối xứng
1. Bắt những số nếu là nhỏ hơn 0 thì false
2. Bắt những số có kết thức bằng 0 vì đến kết thúc bằng 0 thì trong số nguyên không tồn tại những số nào bắt đầu từ 0 (ví dụ: 010, 020)
3. Nhưng nếu đó là số 0 đứng 1 mình thì 0 là vẫn được tính là số đối xứng
4. Tạo một biến gốc để lưu trữ giá trị tham số gốc
5. Tạo một biến đảo ngược có giá trị là số đảo ngược của biến gốc
6. Bắt đầu vòng lặp cho đến dừng khi (biến gốc nhỏ hơn biến đảo ngược)
    6.1 tính lấy số dư: 1221 % 10 = 1 (phần dư số cuối)
    6.2 reserved = (reserved * 10) + phần dư của biến gốc
    6.3 biến gốc = biến gốc / 10 (loại bỏ số cuối)
    --lặp đi lặp lại cho đến biến gốc nhỏ hơn biến đảo ngược (reserved)
7. So sánh biến gốc với biến đảo ngược
    Nếu biến gốc bằng với biến đảo ngược thì trả về true, ngược lại trả về false
    Lưu ý: Nếu biến gốc có số chữ số lẻ, thì cần bỏ qua chữ số giữa khi so sánh nên cần thêm điều kiện (biến gốc == biến đảo ngược / 10)

----
Bài 13: Convert số la mã sang số
1. xác định biến s là tham số chuỗi truyền vào là chuỗi
2. tạo một cuốn sách phiên dịch HashMap rỗng
3. tạo một biến total để lưu trữ Tổng
4. bắt đầu vòng lặp để duyệt từng char từ trái sang phải
    Ký tự 1 (M=1000), bên phải là C=100. 1000 > 100 -> Cộng 1000.
    Ký tự 2 (C=100), bên phải là M=1000. 100 < 1000 -> Trừ 100.
    Ký tự 3 (M=1000), bên phải là X=10. 1000 > 10 -> Cộng 1000.
    Ký tự 4 (X=10), bên phải là C=100. 10 < 100 -> Trừ 10.
    Ký tự 5 (C=100), bên phải là I=1. 100 > 1 -> Cộng 100.
    Ký tự 6 (I=1), bên phải là V=5. 1 < 5 -> Trừ 1.
    Ký tự 7 (V=5), đứng cuối cùng -> Cộng 5.Tổng kết quả: $1000 - 100 + 1000 - 10 + 100 - 1 + 5 = 1994.

---
Bài 14: Lấy tiền tố dài nhất
1. Lấy phần tử đầu của mảng và xem nó là tiền tố dài nhất
2. Duyệt qua các phần tử còn lại trong mảng và so sánh với tiền tố hiện tại và gọt dần nếu khớp
    flower -> flow
    lấy f trong 'flow' rồi xác định 'flower'.start_with('f')
    Lần lượt cho đến khi 'e' và 'r' không khớp thì gọt bỏ
    Tiếp tục lấy flow rồi xác định với 'flight'
3. Nếu cuối cùng bị gọt hết thì trả về chuỗi rỗng