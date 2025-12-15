def get_weather(location: str, units: str = "celsius") -> str:
    """
    Get current weather for a location.
    
    Args:
        location: The city or location to get weather for
        units: Temperature units - either 'celsius' or 'fahrenheit'
        
    Returns:
        str: A description of the current weather
    """
    # This is a mock implementation
    return f"The weather in {location} is sunny and 22 degrees {units}"