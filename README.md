```markdown
# Student Management System API

## Base URL
```

http://localhost:3000

```

## Authentication

### Register Admin
```bash
curl -X POST http://localhost:3000/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "email": "admin@test.com",
    "password": "admin123",
    "full_name": "System Admin"
  }'
```

Login

```bash
curl -X POST http://localhost:3000/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "admin@test.com",
    "password": "admin123"
  }'
```

Save the token from login. Use: Authorization: Bearer YOUR_TOKEN

---

Students

Create Student

```bash
curl -X POST http://localhost:3000/students \
  -H "Authorization: Bearer YOUR_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "student_id": "STU001",
    "full_name": "John Doe",
    "email": "john@example.com",
    "phone": "1234567890",
    "course": "Computer Science",
    "status": "Active"
  }'
```

Get All Students

```bash
curl -X GET "http://localhost:3000/students" \
  -H "Authorization: Bearer YOUR_TOKEN"
```

Get Student by ID

```bash
curl -X GET "http://localhost:3000/students/STU001" \
  -H "Authorization: Bearer YOUR_TOKEN"
```

Search by Name

```bash
curl -X GET "http://localhost:3000/students?name=John" \
  -H "Authorization: Bearer YOUR_TOKEN"
```

Search by Course

```bash
curl -X GET "http://localhost:3000/students?course=Computer%20Science" \
  -H "Authorization: Bearer YOUR_TOKEN"
```

Update Student

```bash
curl -X PUT "http://localhost:3000/students/STU001" \
  -H "Authorization: Bearer YOUR_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "full_name": "John Updated",
    "status": "Inactive"
  }'
```

Delete Student

```bash
curl -X DELETE "http://localhost:3000/students/STU001" \
  -H "Authorization: Bearer YOUR_TOKEN"
```

---

Courses

Create Course

```bash
curl -X POST http://localhost:3000/courses \
  -H "Authorization: Bearer YOUR_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "course_code": "CS101",
    "course_name": "Introduction to Programming",
    "lecturer": "Dr. Smith",
    "duration": "12 weeks",
    "description": "Learn basic programming"
  }'
```

Get All Courses

```bash
curl -X GET "http://localhost:3000/courses" \
  -H "Authorization: Bearer YOUR_TOKEN"
```

Get Course by Code

```bash
curl -X GET "http://localhost:3000/courses/CS101" \
  -H "Authorization: Bearer YOUR_TOKEN"
```

Search by Name

```bash
curl -X GET "http://localhost:3000/courses?course_name=Programming" \
  -H "Authorization: Bearer YOUR_TOKEN"
```

Search by Lecturer

```bash
curl -X GET "http://localhost:3000/courses?lecturer=Smith" \
  -H "Authorization: Bearer YOUR_TOKEN"
```

Update Course

```bash
curl -X PUT "http://localhost:3000/courses/CS101" \
  -H "Authorization: Bearer YOUR_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "course_name": "Advanced Programming",
    "duration": "14 weeks"
  }'
```

Delete Course

```bash
curl -X DELETE "http://localhost:3000/courses/CS101" \
  -H "Authorization: Bearer YOUR_TOKEN"
```

---

Attendance

Mark Attendance

```bash
curl -X POST http://localhost:3000/attendance \
  -H "Authorization: Bearer YOUR_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "student_id": "STUDENT_UUID",
    "course_code": "CS101",
    "date": "2024-01-15",
    "status": true
  }'
```

Get All Attendance

```bash
curl -X GET "http://localhost:3000/attendance" \
  -H "Authorization: Bearer YOUR_TOKEN"
```

Get by Student

```bash
curl -X GET "http://localhost:3000/attendance?student_id=STUDENT_UUID" \
  -H "Authorization: Bearer YOUR_TOKEN"
```

Get by Course

```bash
curl -X GET "http://localhost:3000/attendance?course_code=CS101" \
  -H "Authorization: Bearer YOUR_TOKEN"
```

Get Summary

```bash
curl -X GET "http://localhost:3000/attendance/summary/STUDENT_UUID/CS101" \
  -H "Authorization: Bearer YOUR_TOKEN"
```

Get by Date

```bash
curl -X GET "http://localhost:3000/attendance/date/2024-01-15" \
  -H "Authorization: Bearer YOUR_TOKEN"
```

---

Dashboard

Get Stats

```bash
curl -X GET "http://localhost:3000/dashboard/stats" \
  -H "Authorization: Bearer YOUR_TOKEN"
```

Weekly Attendance

```bash
curl -X GET "http://localhost:3000/dashboard/weekly-attendance" \
  -H "Authorization: Bearer YOUR_TOKEN"
```

Smart Alerts

```bash
curl -X GET "http://localhost:3000/dashboard/alerts" \
  -H "Authorization: Bearer YOUR_TOKEN"
```

Upcoming Tasks

```bash
curl -X GET "http://localhost:3000/dashboard/tasks" \
  -H "Authorization: Bearer YOUR_TOKEN"
```

---

Profile

Get Profile

```bash
curl -X GET "http://localhost:3000/profile" \
  -H "Authorization: Bearer YOUR_TOKEN"
```

Update Profile

```bash
curl -X PUT "http://localhost:3000/profile" \
  -H "Authorization: Bearer YOUR_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "full_name": "Updated Name",
    "email": "newemail@test.com"
  }'
```

Change Password

```bash
curl -X POST "http://localhost:3000/profile/change-password" \
  -H "Authorization: Bearer YOUR_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "current_password": "admin123",
    "new_password": "newpassword123"
  }'
```

---

Notices

Create Notice

```bash
curl -X POST http://localhost:3000/notices \
  -H "Authorization: Bearer YOUR_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Welcome to New Semester",
    "content": "The new semester starts on January 15th"
  }'
```

Get All Notices

```bash
curl -X GET "http://localhost:3000/notices" \
  -H "Authorization: Bearer YOUR_TOKEN"
```

Get Notice by ID

```bash
curl -X GET "http://localhost:3000/notices/NOTICE_ID" \
  -H "Authorization: Bearer YOUR_TOKEN"
```

Update Notice

```bash
curl -X PUT "http://localhost:3000/notices/NOTICE_ID" \
  -H "Authorization: Bearer YOUR_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Updated Title",
    "content": "Updated content"
  }'
```

Delete Notice

```bash
curl -X DELETE "http://localhost:3000/notices/NOTICE_ID" \
  -H "Authorization: Bearer YOUR_TOKEN"
```

---

Reports

Student Report

```bash
curl -X GET "http://localhost:3000/reports/students" \
  -H "Authorization: Bearer YOUR_TOKEN"
```

Attendance Report

```bash
curl -X GET "http://localhost:3000/reports/attendance" \
  -H "Authorization: Bearer YOUR_TOKEN"
```

Course Report

```bash
curl -X GET "http://localhost:3000/reports/course?course_code=CS101" \
  -H "Authorization: Bearer YOUR_TOKEN"
```

Date Range Report

```bash
curl -X GET "http://localhost:3000/reports/date-range?start_date=2024-01-01&end_date=2024-12-31" \
  -H "Authorization: Bearer YOUR_TOKEN"
```

---

Quick Test Script

Save as test_api.sh:

```bash
#!/bin/bash

# Login
TOKEN=$(curl -s -X POST http://localhost:3000/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email": "admin@test.com", "password": "admin123"}' \
  | grep -o '"token":"[^"]*"' | cut -d'"' -f4)

echo "Token: $TOKEN"

# Test Dashboard
echo -e "\n=== Dashboard Stats ==="
curl -X GET "http://localhost:3000/dashboard/stats" \
  -H "Authorization: Bearer $TOKEN"

# Test Students
echo -e "\n=== All Students ==="
curl -X GET "http://localhost:3000/students" \
  -H "Authorization: Bearer $TOKEN"

# Test Courses
echo -e "\n=== All Courses ==="
curl -X GET "http://localhost:3000/courses" \
  -H "Authorization: Bearer $TOKEN"

# Test Notices
echo -e "\n=== All Notices ==="
curl -X GET "http://localhost:3000/notices" \
  -H "Authorization: Bearer $TOKEN"

# Test Reports
echo -e "\n=== Student Report ==="
curl -X GET "http://localhost:3000/reports/students" \
  -H "Authorization: Bearer $TOKEN"
```

---

Environment Variables (.env)

```env
DATABASE_URL=postgres://postgres@localhost/student_management
JWT_SECRET=your_super_secret_jwt_key_change_this
SERVER_HOST=127.0.0.1
SERVER_PORT=3000
```

Run the Server

```bash
cargo run
```

Server runs at: http://localhost:3000

```
## Added academic and finance modules

This backend now includes the missing College Management System modules required for SRS coverage:

### Timetable Management
- `POST /timetables`
- `GET /timetables`
- `GET /timetables/{id}`
- `PUT /timetables/{id}`
- `DELETE /timetables/{id}`

Example body:
```json
{
  "course_code": "BN332",
  "day_of_week": "Monday",
  "start_time": "09:00:00",
  "end_time": "11:00:00",
  "room": "Room 201",
  "lecturer": "Dr Smith",
  "effective_date": "2026-06-04"
}
```

### Assessment / Grading
- `POST /assessments`
- `GET /assessments`
- `GET /assessments/{id}`
- `PUT /assessments/{id}`
- `DELETE /assessments/{id}`

Example body:
```json
{
  "course_code": "BN332",
  "title": "Major Project",
  "assessment_type": "Project",
  "max_marks": 100,
  "weight_percentage": 50,
  "due_date": "2026-06-07",
  "description": "Enterprise web system project"
}
```

### Grades and GPA
- `POST /grades`
- `GET /grades`
- `GET /grades/{id}`
- `PUT /grades/{id}`
- `DELETE /grades/{id}`
- `GET /grades/gpa/{student_id}`

Example body:
```json
{
  "student_id": "PUT_STUDENT_UUID_HERE",
  "assessment_id": "PUT_ASSESSMENT_UUID_HERE",
  "marks_obtained": 82,
  "feedback": "Very good work"
}
```

### Fee Management
- `POST /fees`
- `GET /fees`
- `GET /fees/{id}`
- `PUT /fees/{id}`
- `DELETE /fees/{id}`
- `GET /fees/summary/{student_id}`

Example body:
```json
{
  "student_id": "PUT_STUDENT_UUID_HERE",
  "fee_type": "Tuition Fee",
  "amount": 2500,
  "paid_amount": 1000,
  "due_date": "2026-06-30",
  "notes": "First instalment received"
}
```

All new routes use the same authentication middleware as the existing protected routes and run on `localhost:3000` with no `/api` prefix.
