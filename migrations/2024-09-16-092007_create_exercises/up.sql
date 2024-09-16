-- Your SQL goes here
CREATE TABLE exercises (
    id SERIAL PRIMARY KEY,          -- Auto-incrementing ID
    name VARCHAR(255) NOT NULL,     -- Name of the exercise
    impact VARCHAR(255) NOT NULL,             -- Impact type (e.g., low, high)
    level VARCHAR(255) NOT NULL,              -- Difficulty level (e.g., beginner, advanced)
    description TEXT NOT NULL,               -- Detailed description of the exercise
    video TEXT[] NOT NULL,             -- Link to the exercise video
    male_weight VARCHAR(255) NOT NULL,        -- Suggested male weight range
    female_weight VARCHAR(255) NOT NULL,       -- Suggested female weight range
    musculature_id INTEGER NOT NULL REFERENCES musculature(id)
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
5. Khi lên tạ chú ý không duỗi thẳng cánh tay, luôn giữ 1 góc cong nhẹ ở cùi chỏ', '{"https://gymlab.vn/wp-content/uploads/2023/03/720.mp4"}', 'https://gymlab.vn/wp-content/uploads/2023/03/incline-bench-press.png', 'https://gymlab.vn/wp-content/uploads/2023/03/incline-bench-press-2.png', 1);
