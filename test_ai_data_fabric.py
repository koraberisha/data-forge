from ai_data_fabric import AIDataFabric

def test_ai_data_fabric():
    fabric = AIDataFabric()
    data = [1.0, 2.0, 3.0, 4.0, 5.0]
    fabric.add_data_source("test_data", data, [5])

    sources = fabric.list_data_sources()
    print("Data sources:", sources)

    node = fabric.get_data_source("test_data")
    print("Retrieved data:", node.data)  # Accessing the data attribute directly

    def double(pyarray):
        return pyarray * 2

    try:
        fabric.transform("test_data", double)
    except Exception as e:
        print(f"Error during transformation: {e}")

    transformed_node = fabric.get_data_source("test_data")
    print("Transformed data:", transformed_node.data)  # Accessing the data attribute directly

    fabric.remove_data_source("test_data")
    print("Data sources after removal:", fabric.list_data_sources())

if __name__ == "__main__":
    test_ai_data_fabric()

