// 定义订单组件特征
trait OrderComponent {
    fn execute(&self, order_id: u32, quantity: u32) -> bool;
}

// 基础订单执行器
struct BasicOrderExecutor;

impl OrderComponent for BasicOrderExecutor {
    fn execute(&self, order_id: u32, quantity: u32) -> bool {
        println!("执行基础订单：ID = {}，数量 = {}", order_id, quantity);
        true
    }
}

// 装饰器基础特征
// struct OrderDecorator {
//     wrapped: Box<dyn OrderComponent>,
// }

// 风控检查装饰器
struct RiskCheckDecorator {
    component: Box<dyn OrderComponent>,
    risk_limit: u32,
}

impl OrderComponent for RiskCheckDecorator {
    fn execute(&self, order_id: u32, quantity: u32) -> bool {
        if quantity > self.risk_limit {
            println!("风控检查失败：订单数量 {} 超过限制 {}", quantity, self.risk_limit);
            return false;
        }
        self.component.execute(order_id, quantity)
    }
}

// 日志记录装饰器
struct LoggingDecorator {
    component: Box<dyn OrderComponent>,
}

impl OrderComponent for LoggingDecorator {
    fn execute(&self, order_id: u32, quantity: u32) -> bool {
        println!("开始执行订单：ID = {}，数量 = {}", order_id, quantity);
        let result = self.component.execute(order_id, quantity);
        println!("订单执行完成：ID = {}，结果 = {}", order_id, result);
        result
    }
}

fn main() {
    let order_id = 1001;
    let quantity = 500;

    // 创建基础执行器
    let basic_executor = Box::new(BasicOrderExecutor);

    // 添加风控检查
    let risk_checker = Box::new(RiskCheckDecorator {
        component: basic_executor,
        risk_limit: 1000,
    });

    // 添加日志记录
    let logger = Box::new(LoggingDecorator {
        component: risk_checker,
    });

    // 执行订单
    logger.execute(order_id, quantity);
}