# 데코레이터 (Decorator)

- **목표:** 기존 객체를 변경하지 않고, 새로운 기능을 가진 데코레이터 객체로 감싸서(Wrapping) 기능을 동적으로 확장하는 데코레이터 패턴을 구현합니다.
- **조건:** Rust에서는 인터페이스 역할을 하는 `trait`를 정의하고, 데코레이터 구조체가 해당 트레이트를 구현하는 다른 객체를 내부에 소유(Composition)하면서 동일한 트레이트를 중첩하여 구현하는 방식으로 설계합니다.

### [연습 문제] 데코레이터 패턴 구현

기본 커피(Coffee)에 우유(Milk)나 설탕(Sugar) 등의 토핑을 동적으로 추가하며 가격(Cost)을 계산하는 시스템을 데코레이터 패턴으로 구현하세요.

1. `Coffee` 트레이트는 `get_cost` 메서드를 가집니다.
2. `SimpleCoffee` 구조체는 가장 기본이 되는 구체적인 컴포넌트(Concrete Component)이며, 기본 가격은 `2000`입니다.
3. `MilkDecorator`와 `SugarDecorator` 구조체는 각각 내부에 `Box<dyn Coffee>`를 소유하는 데코레이터입니다. 기존 커피 가격에 자신들의 토핑 가격(`Milk: 500`, `Sugar: 200`)을 더하여 반환합니다.

```rust
// 1. 공통 인터페이스 (Component)
trait Coffee {
    fn get_cost(&self) -> u32;
}

// 2. 구체적인 컴포넌트 (Concrete Component)
struct SimpleCoffee;
impl Coffee for SimpleCoffee {
    fn get_cost(&self) -> u32 {
        2000
    }
}

// 3. 구체적인 데코레이터 1 (Concrete Decorator - 우유 추가)
struct MilkDecorator {
    coffee: Box<dyn Coffee>,
}

impl MilkDecorator {
    fn new(coffee: Box<dyn Coffee>) -> Self {
        MilkDecorator { coffee }
    }
}

// MilkDecorator에 Coffee 트레이트를 구현하세요.
// get_cost 메서드는 내부 coffee의 가격에 500을 더한 값을 반환해야 합니다.
impl Coffee for MilkDecorator {
    // 이 부분을 구현하세요.
}

// 4. 구체적인 데코레이터 2 (Concrete Decorator - 설탕 추가)
struct SugarDecorator {
    coffee: Box<dyn Coffee>,
}

impl SugarDecorator {
    fn new(coffee: Box<dyn Coffee>) -> Self {
        SugarDecorator { coffee }
    }
}

// SugarDecorator에 Coffee 트레이트를 구현하세요.
// get_cost 메서드는 내부 coffee의 가격에 200을 더한 값을 반환해야 합니다.
impl Coffee for SugarDecorator {
    // 이 부분을 구현하세요.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decorator_pattern() {
        // 1. 기본 커피 (2000)
        let coffee: Box<dyn Coffee> = Box::new(SimpleCoffee);
        assert_eq!(coffee.get_cost(), 2000);

        // 2. 우유 추가 (2000 + 500 = 2500)
        let milk_coffee: Box<dyn Coffee> = Box::new(MilkDecorator::new(coffee));
        assert_eq!(milk_coffee.get_cost(), 2500);

        // 3. 설탕 추가 (2500 + 200 = 2700)
        let sugar_milk_coffee: Box<dyn Coffee> = Box::new(SugarDecorator::new(milk_coffee));
        assert_eq!(sugar_milk_coffee.get_cost(), 2700);
    }
}
```

[현재 단계: 구조 패턴 - 데코레이터]