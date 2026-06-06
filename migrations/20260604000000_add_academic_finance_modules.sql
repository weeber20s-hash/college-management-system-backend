-- Timetable, assessments/grades/GPA, and fee-management modules

CREATE TABLE IF NOT EXISTS timetables (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    course_code TEXT NOT NULL REFERENCES courses(course_code) ON DELETE CASCADE,
    day_of_week TEXT NOT NULL,
    start_time TIME NOT NULL,
    end_time TIME NOT NULL,
    room TEXT,
    lecturer TEXT,
    effective_date DATE
);

CREATE TABLE IF NOT EXISTS assessments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    course_code TEXT NOT NULL REFERENCES courses(course_code) ON DELETE CASCADE,
    title TEXT NOT NULL,
    assessment_type TEXT NOT NULL,
    max_marks DOUBLE PRECISION NOT NULL CHECK (max_marks > 0),
    weight_percentage DOUBLE PRECISION NOT NULL CHECK (weight_percentage >= 0),
    due_date DATE,
    description TEXT
);

CREATE TABLE IF NOT EXISTS grades (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    student_id UUID NOT NULL REFERENCES students(id) ON DELETE CASCADE,
    assessment_id UUID NOT NULL REFERENCES assessments(id) ON DELETE CASCADE,
    marks_obtained DOUBLE PRECISION NOT NULL CHECK (marks_obtained >= 0),
    grade_letter TEXT,
    feedback TEXT,
    UNIQUE(student_id, assessment_id)
);

CREATE TABLE IF NOT EXISTS fees (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    student_id UUID NOT NULL REFERENCES students(id) ON DELETE CASCADE,
    fee_type TEXT NOT NULL,
    amount DOUBLE PRECISION NOT NULL CHECK (amount >= 0),
    paid_amount DOUBLE PRECISION NOT NULL DEFAULT 0 CHECK (paid_amount >= 0),
    due_date DATE,
    status TEXT NOT NULL DEFAULT 'Unpaid',
    payment_date DATE,
    notes TEXT
);

CREATE INDEX IF NOT EXISTS idx_timetables_course_code ON timetables(course_code);
CREATE INDEX IF NOT EXISTS idx_assessments_course_code ON assessments(course_code);
CREATE INDEX IF NOT EXISTS idx_grades_student_id ON grades(student_id);
CREATE INDEX IF NOT EXISTS idx_fees_student_id ON fees(student_id);
