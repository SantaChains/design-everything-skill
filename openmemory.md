设计结构  color_
方向名称_

选择文本结构

更新设计关系 tsv sqllite
kdl 

AskUserQuestion

时间观

模糊 向量 语意分析 NLP 词云 分词




----



以 深刻 犀利 大胆 前卫 为风格;
完全满足用户要求,最佳实践为导向 但最强实现为目标;
对要设计的行业的专业知识 深度学习 搜索 分析, 论文 网站 找到;
从美术,视听 效果的专业人士的艺术思维 视角构思创意 亮点;
行业设计规范为限制,不可行的不能做 查找的资料要绝对正确,经过验证(不是404的链接,最新官方的api)
用户有错误直接和歧义,先AskUserQuestion,不要盲目执行和编造,
最忌讳直接覆盖已有文件,优先替换需要修改的内容.忌讳遗漏细节,忌讳错误的内容直接执行
不理解的概念要 从输出错误 对相关的内容进行推断和模糊搜索

代码检验
代码质量检查 删除死代码废弃代码 排除所有隐患 修正所有错误 验证代码间通信 整体要工作严密 接口与交换统一一致 前后端要 文档或注释的语义对代码逻辑不一致的地方要AskUserQuestion,在从设计方面寻找最合理解释和原因

项目的设置要考虑丰富 操作流程要符合人类直觉习惯 
交互 要贴合实际使用


输出风格要思辨 理性








---


我想结合kdl配置语言,tsv数据结构 sqlite存储数据库关系 py脚本(或者rust)
配合 美术设计从业者的创意视角 涉及领域专业知识 
提高[[OTHER-SKILLS]]三个skill的调用倾向
优秀的文本素材 (比如从优秀文学中提炼,动作设计,姿势设计,人物外貌设计)
    - 已配置文学素材库: literature_library.kdl
    - 国际资源: Project Gutenberg (75,000+ 公有领域作品)
    - 国内资源: 书格、国家图书馆古籍平台、国学大师等
    - 详细报告: 公有领域文学素材库研究报告.md
    - 存储格式: TSV (与项目架构一致)
    - 素材文件: 
        * data/tsv/literature_action.tsv (动作设计素材, 8条)
        * data/tsv/literature_pose.tsv (姿势设计素材, 7条)
        * data/tsv/literature_character.tsv (人物外貌素材, 7条)
        * data/tsv/literature_emotion.tsv (情感表达素材, 6条)
        * data/tsv/literature_action_foreign.tsv (外国动作设计素材, 5条)
        * data/tsv/literature_pose_foreign.tsv (外国姿势设计素材, 5条)
        * data/tsv/literature_character_foreign.tsv (外国人物外貌素材, 6条)
        * data/tsv/literature_emotion_foreign.tsv (外国情感表达素材, 5条)
    - 素材总数: 28条(中国) + 21条(外国) = 49条, 平均质量: 9.2/10
    - 外国经典名著: 详细清单见"外国公有领域经典名著完整清单.md"
        * 西方: 莎士比亚、狄更斯、雨果、大仲马、托尔斯泰、陀思妥耶夫斯基等
        * 美国: 马克·吐温、梅尔维尔、杰克·伦敦等
        * 日本: 紫式部《源氏物语》、松尾芭蕉俳句等
        * 拉丁美洲: 马尔克斯、博尔赫斯等
        * 其他: 契诃夫、薄伽丘、荷马等


我这个skill design everything项目想要结合spec prompt agent role设计在ai模块:
ai对 词汇的 理解能力 和敏感程度 
噪声 信噪比 忌讳抽象概念词
ai参数 温度、top_p、重复惩罚、上下文长度、风格强度
