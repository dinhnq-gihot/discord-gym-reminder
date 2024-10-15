-- Your SQL goes here
CREATE TABLE exercises (
    id SERIAL PRIMARY KEY,              -- Auto-incrementing ID
    name VARCHAR(255) NOT NULL,         -- Name of the exercise
    impact VARCHAR(255) NOT NULL,       -- Impact type (e.g., low, high)
    level VARCHAR(255) NOT NULL,              -- Difficulty level (e.g., beginner, advanced)
    description TEXT NOT NULL,               -- Detailed description of the exercise
    video TEXT[] NOT NULL,             -- Link to the exercise video
    male_weight VARCHAR(255) NOT NULL,        -- Suggested male weight range
    female_weight VARCHAR(255) NOT NULL,       -- Suggested female weight range
    musculature_id INTEGER NOT NULL REFERENCES musculatures(id)
);

INSERT INTO exercises (name, impact, level, description, video, male_weight, female_weight, musculature_id)
VALUES
('ĐẨY TẠ ĐƠN TRÊN GHẾ NGHIÊNG - INCLINE DUMBBELL PRESS - Biến thể 1', 'Ngực trên', 'mới bắt đầu', '1. Nằm trên ghế nghiêng 30-45 độ, chân đạp xuống sàn để nhấc nhẹ hông lên
2. 2 cánh tay để 1 góc 45-60 độ với cơ thể
3. Lưng có thể Arch (cong) nhẹ hoặc nằm thẳng đều được
4. Lên và Hạ tạ từ từ và cảm nhận cơ bắp
5. Khi lên tạ chú ý không duỗi thẳng cánh tay, luôn giữ 1 góc cong nhẹ ở cùi chỏ', '{"https://gymlab.vn/wp-content/uploads/2023/03/720-Cach-tap-incline-dumbbell-press-ngang-GymLab.mp4", "https://gymlab.vn/wp-content/uploads/2023/03/720-Cach-tap-incline-dumbbell-press-nghieng-GymLab.mp4"}', 'https://gymlab.vn/wp-content/uploads/2023/03/dumbbell-incline-press.png', 'https://gymlab.vn/wp-content/uploads/2023/03/dumbbell-incline-press-2.png', 1),
('ĐẨY TẠ ĐÒN TRÊN GHẾ NGHIÊNG - INCLINE BENCH PRESS- Biến thể 2', 'Ngực trên', 'trung bình', '1. Nằm trên ghế nghiêng 30-45 độ, chân đạp xuống sàn để nhấc nhẹ hông lên
2. Tay mở rộng gấp 1.5 lần độ rộng vai. Gồng core chắc chắn
3. 2 cánh tay để 1 góc 45-60 độ với cơ thể
4. Lên và Hạ tạ từ từ và cảm nhận cơ bắp
5. Khi lên tạ chú ý không duỗi thẳng cánh tay, luôn giữ 1 góc cong nhẹ ở cùi chỏ', '{"https://gymlab.vn/wp-content/uploads/2023/03/720.mp4"}', 'https://gymlab.vn/wp-content/uploads/2023/03/incline-bench-press.png', 'https://gymlab.vn/wp-content/uploads/2023/03/incline-bench-press-2.png', 1),
('MÁY ĐẨY NGỰC - CHEST PRESS MACHINE - Biến thể 1', 'Ngực giữa', 'mới bắt đầu', '1. Chỉnh độ cao ghế để tay cầm ngang với ngực.
2. Ưỡn ngực, ép chặt lưng vào ghế.
3. Cánh tay tạo với ngực 1 góc 45-60 độ.
4. Thực hiện động tác đẩy, đồng thời ép chặt cơ ngực.
5. Tập với tốc độ từ từ và cảm nhận cơ bắp', '{"https://gymlab.vn/wp-content/uploads/2023/03/720-Cach-tap-may-day-nguc-nghieng-Gymlab.mp4"}', 'https://gymlab.vn/wp-content/uploads/2023/03/May-day-nguc-1.png', 'https://gymlab.vn/wp-content/uploads/2023/03/May-day-nguc-3.png', 2),
('BAY NGỰC VỚI DÂY CÁP TỪ TRÊN XUỐNG - HIGH TO LOW CABLE CHEST FLY - Biến thể 1', 'Ngực dưới', 'trung bình', '1. Chỉnh độ cao 2 bên dây cáp trên đầu, gắn tay cầm đơn vào từng bên.
2. Kéo dây cáp xuống và tiến người lên 1 bước chân
3. Nghiêng nhẹ người (lưng) về phía trước để tạo điểm tựa chắc cho cơ thể
4. Khi đẩy tay hướng xuống dưới và ép chặt cơ ngực giúp cảm nhận cơ tốt nhất
5. Lên và Hạ tạ từ từ và cảm nhận cơ bắp', '{"https://gymlab.vn/wp-content/uploads/2023/03/720-Cach-tap-ep-nguc-voi-day-cap-tu-cao-xuong-thap-Ngang-Gymlab-1.mp4"}', 'https://gymlab.vn/wp-content/uploads/2023/03/Bay-nguc-voi-day-cap-tu-tren-xuong-1.png', 'https://gymlab.vn/wp-content/uploads/2023/03/Bay-nguc-voi-day-cap-tu-tren-xuong-3.png', 3),
('MÁY ÉP NGỰC - CHEST FLY MACHINE - Biến thể 1', 'ép ngực', 'mới bắt đầu', '1. Chỉnh độ cao ghế để tay cầm thấp hơn vai 10cm.
2. Mở ngực, Arch nhẹ lưng, siết chặt bả vai vào ghế.
3. 2 tay hơi cong nhẹ để tạo lực ép tốt nhất.
4. Khi ép vào vị trí sâu nhất, duỗi thẳng tay đồng thời ép chặt cơ ngực.
5. Tập với tốc độ từ từ và cảm nhận cơ bắp', '{"https://gymlab.vn/wp-content/uploads/2023/03/720-Cach-tap-may-ep-nguc-Ngang-Gymlab.mp4"}', 'https://gymlab.vn/wp-content/uploads/2023/03/May-ep-nguc-1.png', 'https://gymlab.vn/wp-content/uploads/2023/03/May-ep-nguc-3.png', 4),
('KÉO LƯNG XÔ VỚI MÁY- LATS PULLDOWN - Biến thể 1', 'Lưng xô', 'mới bắt đầu', '1. Đặt tay lên thanh đòn, độ rộng bằng 1.5 vai.
2. Giữ thẳng lưng, gồng chắc vai.
3. Xoay nhẹ cùi chỏ về phía trước để tác động tốt nhất vào cơ lưng
4. Khi lên, giãn nhẹ cơ xô để tối ưu cả chuyện động co và giãn
5. Thực hiện động tác từ từ và cảm nhận cơ bắp ở cả chiều lên và xuống.', '{"https://gymlab.vn/wp-content/uploads/2023/03/720-Cach-tap-bai-lat-pull-down-ngang-Gymlab.mp4", "https://gymlab.vn/wp-content/uploads/2023/03/720-Cach-tap-bai-lat-pull-down-nghieng-Gymlab.mp4"}', 'https://gymlab.vn/wp-content/uploads/2023/03/May-keo-xo-1.png', 'https://gymlab.vn/wp-content/uploads/2023/03/May-keo-xo-3.png', 5);
