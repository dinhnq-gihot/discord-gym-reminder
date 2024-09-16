-- Your SQL goes here
CREATE TABLE musculature (
    id SERIAL PRIMARY KEY,          -- Auto-incrementing ID
    name VARCHAR(255) NOT NULL,
    note TEXT
);

INSERT INTO musculature (name, note) VALUES
('NGỰC TRÊN', 'Chỉ nên tập 1 biến thể'), --1
('ĐẨY NGỰC GIỮA', 'Chỉ nên tập 1 biến thể'), --2
('ÉP NGỰC DƯỚI', 'Chỉ nên tập 1 biến thể'), --3
('ÉP NGỰC', 'Chỉ nên tập 1 biến thể'), --4
('LƯNG XÔ', 'Chỉ nên tập 1 biến thể'), --5
('LƯNG GIỮA', 'Chỉ nên tập 1 biến thể'), --6
('TOÀN BỘ LƯNG', 'Chỉ nên tập 1 biến thể'), --7
('LƯNG DƯỚI', NULL), --8
('TOÀN BỘ CHÂN', 'Chỉ nên tập 1 biến thể'), --9
('MÔNG', 'Chỉ nên tập 1 biến thể'), --10
('ĐÙI SAU', NULL), --11
('ĐÙI TRƯỚC', 'Có thể tập cả 2 biến thể'), --12
('BẮP CHÂN', 'Chỉ nên tập 1 biến thể'), --13
('BỤNG', 'Tập cả 2 biến thể'), --14
('VAI TRƯỚC', NULL), --15 
('ĐẨY VAI GIỮA', 'Chỉ nên tập 1 biến thể'), --16
('BAY VAI GIỮA', 'Chỉ nên tập 1 biến thể'), --17
('VAI SAU', 'Chỉ nên tập 1 biến thể'), --18
('CẦU VAI', 'Chỉ nên tập 1 biến thể'), --19
('TAY TRƯỚC', 'Chỉ nên tập 1 biến thể'), --20
('CẲNG TAY', 'Chỉ nên tập 1 biến thể'), --21
('TAY SAU', 'Chỉ nên tập 1 biến thể'), --22
('TAY TRƯỚC', 'Chỉ nên tập 1 biến thể'); --23