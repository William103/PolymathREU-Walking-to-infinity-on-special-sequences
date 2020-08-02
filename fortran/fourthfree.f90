program test
implicit none
    1   format(1i10)

    integer (kind = 8), parameter :: base = 2, chunk = 10
    double precision :: S
    integer (kind = 8) :: i, len

    open(1, file = "prev.txt")
    write(1,1) 1
    close(1)

    S = 0.5

    do i = 2, 63
        len = next()
        call rename("array.txt", "prev.txt")
        S = S + (real(len) / 2.0 ** i)
        print *, "ITERATION", i
        print *, "NUMBER   ", len
        print *, "SUM      ", S
        print *, ""
        call flush()
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

function next()
implicit none
    1   format(1i10)

    integer (kind = 8), dimension (:), allocatable :: temp, temp2
    integer (kind = 8) :: current, next
    integer :: i, j, templen, ios
    logical :: done

    templen = 0
    next = 0
    done = .false.
    allocate(temp(chunk))

    open(1, file = "array.txt")
    open(2, file = "prev.txt")

    do while (.not. done)
        read(2, 1, iostat = ios) current
        if (ios .ne. 0) then
            done = .true.
            exit
        end if

        temp2 = step(current)
        do i = 1, size(temp2)
            templen = templen + 1
            temp(templen) = temp2(i)
            next = next + 1
            if (templen >= chunk) then
                write(1,1) temp
                call flush(1)
                deallocate(temp)
                allocate(temp(chunk))
                templen = 0
            end if
        end do
        deallocate(temp2)
    end do

    write(1,1) temp(:templen)
    deallocate(temp)

    close(1)
    close(2)

end function next

end program test
