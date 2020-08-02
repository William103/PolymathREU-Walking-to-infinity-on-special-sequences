program test
implicit none
    1   format(20i10)

    integer (kind = 8), parameter :: base = 2
    integer (kind = 8), dimension (:), allocatable :: ls, temp1, temp2
    double precision :: S
    integer (kind = 8) :: i

    allocate(ls(1))
    ls = (/1/)
    S = 0

    do i = 1, 63
        S = S + (real(size(ls)) / real(2 ** i))
        print *, "ITERATION", i
        print *, "NUMBER   ", size(ls)
        print *, "SUM      ", S
        print *, ""
        call flush()
        temp1 = next(ls)
        temp2 = ls
        ls = temp1
        deallocate(temp2)
    end do
    

contains

function is_fourth_free (x)

    integer (kind = 8), intent (in) :: x
    integer (kind = 8)              :: i
    logical :: is_fourth_free

    i = 2
    do while (i * i * i * i <= x)
        if (mod(x, i * i * i * i) == 0) then
            is_fourth_free = .false.
            return
        end if
        i = i + 1
    end do
    is_fourth_free = .true.

end function is_fourth_free

function is_square_free (x)

    integer (kind = 8), intent (in) :: x
    integer (kind = 8)              :: i
    logical :: is_square_free

    i = 2
    do while (i * i <= x)
        if (mod(x, i * i) == 0) then
            is_square_free = .false.
            return
        end if
        i = i + 1
    end do
    is_square_free = .true.

end function is_square_free

function is_prime (x)

    integer (kind = 8), intent (in) :: x
    integer (kind = 8)              :: i
    logical :: is_prime

    if (x < 2 .or. mod(x, 2) == 0) then
        is_prime = .false.
        return
    end if

    i = 3
    do while (i * i <= x)
        if (mod(x, i) == 0) then
            is_prime = .false.
            return
        end if
        i = i + 2
    end do
    is_prime = .true.

end function is_prime

function step (x)
implicit none

    integer (kind = 8), intent (in)                :: x
    integer (kind = 8)                             :: i, t, count
    integer (kind = 8), dimension (:), allocatable :: step
    integer (kind = 8), dimension (base)           :: temp

    count = 0

    do i = 0, base - 1
        t = x * base + i
        if (is_fourth_free(t)) then
            count = count + 1
            temp(count) = t
        end if
    end do

    allocate(step(count))

    do i = 1, count
        step(i) = temp(i)
    end do

end function step

function next (ls)
    1   format(20i10)

    integer (kind = 8), dimension (:), allocatable, intent (in) :: ls
    integer (kind = 8), dimension (:), allocatable :: temp, temp2, next
    integer :: s, i, j, count

    count = 0
    s = size(ls)
    allocate(temp(s * base))

    do i = 1, s
        temp2 = step(ls(i))
        do j = 1, size(temp2)
            temp(j + count) = temp2(j)
        end do
        count = count + size(temp2)
        deallocate(temp2)
    end do

    allocate(next(count))
    do i = 1, count
        next(i) = temp(i)
    end do

    deallocate(temp)

end function next

end program test
