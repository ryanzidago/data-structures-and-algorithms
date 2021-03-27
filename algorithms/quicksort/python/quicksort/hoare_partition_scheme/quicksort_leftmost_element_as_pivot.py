def quicksort(array):
    left_index = 0
    right_index = len(array) - 1
    _quicksort(array, left_index, right_index)


def _quicksort(array, left_index, right_index):
    if left_index >= right_index:
        return

    pivot_index = partition(array, left_index, right_index)
    _quicksort(array, left_index, pivot_index - 1)
    _quicksort(array, pivot_index + 1, right_index)


def partition(array, left_index, right_index):
    pivot_index = left_index
    left_index += 1

    while True:
        while array[left_index] < array[pivot_index]:
            left_index += 1

        while array[right_index] > array[pivot_index] and right_index > 0:
            right_index -= 1

        if left_index >= right_index:
            array[right_index], array[pivot_index] = array[pivot_index], array[right_index]
            return right_index
        else:
            array[right_index], array[left_index] = array[left_index], array[right_index]
            right_index -= 1
